use json::JsonValue;
use reqwest::StatusCode;
use regex::Regex;
use reqwest::header::RANGE;
use std::fs::File;
use youtube::youtube::*;
use requests::requests::*;
use errors::*;

mod youtube;
mod requests;

#[macro_use]
extern crate error_chain;

pub mod errors {
    use std::io;

    error_chain! {
        foreign_links {
            Io(io::Error) #[doc = "Error during IO"];
        }
    }
}


pub async fn get_available_sources(video_url: &str) -> Result<Vec<DownloadSource>> {
    let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
    let caps = url_regex.captures(video_url).chain_err(|| "Video url is not valid")?;
    let video_id = caps.get(1).chain_err(|| "Could not parse video ID from url")?.as_str();

    let body = create_request_body(video_id);

    let client = reqwest::Client::new();
    let res = client.post(YOUTUBE_ENDPOINT).body(body).send().await
        .chain_err(|| "Could not send POST request to Youtube endpoint")?;
    let response_text = res.text().await
        .chain_err(|| "Youtube endpoint didn't return any content")?;

    let response_json = json::parse(&response_text).chain_err(|| "Could not parse JSON response")?;

    let adaptive_formats: Vec<&JsonValue> =
        response_json["streamingData"]["adaptiveFormats"].members().collect();

    adaptive_formats.into_iter().map(|f| f.clone()).map(|f| DownloadSource::try_from(f)).collect()
}

pub async fn download_video(source: DownloadSource, output_file: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let video_url = source.video_url;
    let length = source.content_length;
    let mut output_file = File::create(output_file).chain_err(|| "Could not create output file")?;

    const CHUNK_SIZE: u32 = 10240;

    println!("starting download...");
    for range in PartialRangeIter::new(0, length-1, CHUNK_SIZE)? {
        println!("range: {:#?}", range);
        let response = client.get(&video_url).header(RANGE, range).send().await
            .chain_err(|| "Could not send partial request for video")?;

        let status = response.status();
        println!("status: {status}");
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            bail!("Response status code was not OK or PARTIAL_CONTENT");
        }
        
        let content = response.text().await.chain_err(|| "No video content returned")?;
        std::io::copy(&mut content.as_bytes(), &mut output_file)
            .chain_err(|| "Could not copy video content to output file")?;
    } 
    
    println!("Finished successfuly!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn available_sources_are_returned() {
        let available_sources = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA&pp=ygUMcG91dHNlcyBtcGxl")
            .await.unwrap();
        assert_eq!(16, available_sources.len());
        assert_eq!(4, available_sources.iter().filter(|s| s.mime_type.contains("audio")).count());
    }

    #[tokio::test]
    async fn video_is_downloaded() {
        const OUTPUT_FILE: &str = "download.mp4"; 
        let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA&pp=ygUMcG91dHNlcyBtcGxl")
            .await.unwrap().into_iter().filter(|s| s.mime_type.contains("audio")).next().unwrap();
        let video = download_video(source, OUTPUT_FILE).await;
        assert!(video.is_ok());
        let output_file = File::open(OUTPUT_FILE).unwrap();
        assert!(output_file.metadata().unwrap().len() > 0); 
    }

    #[allow(dead_code)]
    fn mock_download_source() -> DownloadSource {
        DownloadSource {
            video_url: "https://rr1---sn-p5qlsny6.googlevideo.com/videoplayback?expire=1681507452&ei=G3A5ZO-hOeGM_9EPiOWTgA0&ip=54.86.50.139&id=o-AN4SX0Ur7Gs4jbdsUqgyCW8bgVJsAXCMzKxPqDnBBmWa&itag=140&source=youtube&requiressl=yes&mh=fn&mm=31%2C29&mn=sn-p5qlsny6%2Csn-p5qs7nzk&ms=au%2Crdu&mv=m&mvi=1&pl=16&initcwndbps=955000&spc=99c5CXk6Fbuf6M1vS7GfkE1gIyOjDRaHIK4S9CYsnA&vprv=1&mime=audio%2Fmp4&ns=Hc5XID5fFdR4dNR7HWosSe8M&gir=yes&clen=10459587&dur=646.234&lmt=1681389517005368&mt=1681485191&fvip=2&keepalive=yes&fexp=24007246&c=WEB&txp=6318224&n=xQGIkh8QyPnRlEidROe&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cspc%2Cvprv%2Cmime%2Cns%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgdmKjVCLPmqmoKBgvZ7fZwESONhhlI2XaguKr3CQmF7gCIQCB568oWjwXBKnitxGHjPrAR2ASl0x1Whp1wZdmr4Ebhg%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIhAN1ifQomWCkRrFZqImilnbIX_oU4ygH48OUzU5FWYD8BAiB6ZpsdFteotI6BrXZZN8Nf06yxvHvTc2goPah6k2Bnzw%3D%3D".to_string(),
            mime_type: "audio/mp4, codecs=\"mp4a.40.2\"".to_string(),
            quality: "tiny".to_string(),
            content_length: 10459587,
            width: None,
            height: None,
            fps: None,
            quality_label: None,
            bitrate: Some(130769),
            avg_bitrate: 129483,
            audio_quality: Some("AUDIO_QUALITY_MEDIUM".to_string()),
            audio_sample_rate: Some(44100),
            audio_channels: Some(2),
        }    
    }
}

