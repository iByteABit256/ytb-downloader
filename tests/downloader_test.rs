use ytb_downloader::*;
use youtube::youtube::DownloadSource;

#[tokio::test]
async fn available_sources_are_returned() {
    let available_sources = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA")
        .await.unwrap();
    assert_eq!(16, available_sources.len());
    assert_eq!(4, available_sources.iter().filter(|s| s.mime_type.contains("audio")).count());
}

#[tokio::test]
async fn video_is_downloaded() {
    const OUTPUT_FILE: &str = "assets/test.m4a"; 
    const TEST_OUTPUT_FILE: &str = "assets/test-output.m4a"; 

    let output_file = tokio::fs::File::create(OUTPUT_FILE)
        .await.unwrap();
    let test_output_file = tokio::fs::File::open(TEST_OUTPUT_FILE)
        .await.unwrap();

    let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA")
        .await.unwrap().into_iter().filter(|s| s.mime_type.contains("audio")).next().unwrap();

    let video = download_video!(&source, OUTPUT_FILE).await;

    assert!(video.is_ok());

    assert!(output_file.metadata().await.unwrap().len() > 0); 
    assert_eq!(test_output_file.metadata().await.unwrap().len(), output_file.metadata().await.unwrap().len()); 
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

