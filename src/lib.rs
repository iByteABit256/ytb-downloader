//! # ytb-downloader
//!
//! A library for downloading Youtube videos
//!
//! ## Examples
//!
//! ```
//! # #[macro_use] extern crate ytb_downloader;
//! # use ytb_downloader::*;
//! # use errors::*;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
//!     .into_iter().next().unwrap();
//! const OUTPUT_FILE: &str = "download.m4a"; 
//! download_video!(&source, OUTPUT_FILE).await;
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! # #[macro_use] extern crate ytb_downloader;
//! # use ytb_downloader::*;
//! # use errors::*;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
//!     .into_iter().next().unwrap();
//! const OUTPUT_FILE: &str = "download.m4a"; 
//! download_video(&source, OUTPUT_FILE, None).await;
//! # Ok(())
//! # }
//! ```

use errors::*;
use log::{info,debug};
use reqwest::StatusCode;
use reqwest::header::RANGE;
use requests::requests::*;
use youtube::youtube::*;
use json::JsonValue;


/// Functions related to requests to Youtube
pub mod youtube;
mod requests;

#[macro_use]
extern crate error_chain;

/// Error types of ytb-downloader crate
pub mod errors {
    use std::io;

    error_chain! {
        foreign_links {
            Io(io::Error) #[doc = "Error during IO"];
        }

        errors {
            InvalidYoutubeLink {
                description("Could not parse youtube link"),
                display("Could not parse youtube link"),
            } 

            VideoIdEmpty {
                description("Video ID is empty"),
                display("Video ID is empty"),
            } 

            GetRequestError {
                description("Problem encountered during GET request"),
                display("Problem encountered during GET request"),
            }

            PostRequestError {
                description("Problem encountered during POST request"),
                display("Problem encountered during POST request"),
            }

            ResponseStatusError(r: String) {
                description("Response status was not expected"),
                display("Response status {} was not expected", r),
            }

            DownloadError {
                description("Problem encountered while downloading video"),
                display("Problem encountered while downloading video"),
            }

            JsonParseError {
                description("Error while parsing JSON response"),
                display("Error while parsing JSON response"),
            }

            JsonPropertyError(p: String) {
                description("Error while reading JSON property"),
                display("Error while reading JSON property: {}", p),
            }

            FileError(c: String) {
                description("Error with output file"),
                display("Error with output file: {}", c),
            }
        }
    }
}

/// Gets available download sources and formats for a video
///
/// ## Examples
///
/// Gets available sources for video
/// ```
/// # use ytb_downloader::*;
/// # use errors::*;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrw_bea")
///      .await?;
/// # Ok(())
/// # }
/// ```
///
/// Gets available sources with audio format for video
/// ```
/// # use ytb_downloader::*;
/// # use errors::*;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA")
///      .await?.into_iter().filter(|s| s.mime_type.contains("audio")).next().unwrap();
/// # Ok(())
/// # }
/// ```
pub async fn get_available_sources(video_url: &str) -> Result<Vec<DownloadSource>> {
    let video_id = &parse_video_id(video_url).chain_err(|| ErrorKind::InvalidYoutubeLink)?;
    
    if video_id.is_empty() {
        return Err(ErrorKind::VideoIdEmpty)?;
    }

    let body = create_request_body(video_id);
    debug!("Request body created: {}", body);

    let client = reqwest::Client::new();

    info!("Requesting source info for video ID: {}", video_id);
    let res = client.post(YOUTUBE_ENDPOINT).body(body).send().await
        .chain_err(|| ErrorKind::PostRequestError)?;
    debug!("POST response for source info: {:#?}", res);

    let response_text = res.text().await
        .chain_err(|| ErrorKind::PostRequestError)?;

    let response_json = json::parse(&response_text)
        .chain_err(|| ErrorKind::JsonParseError)?;
    info!("Source info was returned.");

    let adaptive_formats: Vec<&JsonValue> =
        response_json["streamingData"]["adaptiveFormats"].members().collect();
    debug!("Adaptive formats: {:#?}", adaptive_formats);

    adaptive_formats.into_iter().map(|f| f.clone()).map(|f| DownloadSource::try_from(f)).collect()
}

/// Downloads a video to an output file given a download source
/// and optionally a chunk size for the partial requests
/// 
/// Downloads a video to the output file 
/// ```
/// # #[macro_use] extern crate ytb_downloader;
/// # use ytb_downloader::*;
/// # use errors::*;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// # let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
/// #   .into_iter().next().unwrap();
/// const OUTPUT_FILE: &str = "download.m4a"; 
/// let video = download_video(&source, OUTPUT_FILE, None).await;
/// # Ok(())
/// # }
/// ```
///
/// Downloads a video to the output file with a chunk size of 10240 bytes
/// ```
/// # #[macro_use] extern crate ytb_downloader;
/// # use ytb_downloader::*;
/// # use errors::*;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// # let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
/// #   .into_iter().next().unwrap();
/// const OUTPUT_FILE: &str = "download.m4a"; 
/// let video = download_video(&source, OUTPUT_FILE, Some(10240)).await;
/// # Ok(())
/// # }
/// ```
pub async fn download_video(source: &DownloadSource, output_file: &str, chunk_size: Option<u32>) -> Result<()> {
    let client = reqwest::Client::new();
    let video_url = &source.video_url;
    let length = source.content_length;
    let mut output_file = tokio::fs::File::create(output_file).await
        .chain_err(|| ErrorKind::FileError("creation".to_string()))?;

    let chunk_size = if chunk_size.is_some() { chunk_size.unwrap() } else { length as u32 };

    debug!("Downloading from: {video_url}");
    info!("Content-Length: {length}");
    info!("Starting download...");
    for range in PartialRangeIter::new(0, length-1, chunk_size)? {
        info!("Requesting range: {:#?}/{}", range, length-1);
        let response = client.get(video_url).header(RANGE, range).send().await
            .chain_err(|| ErrorKind::GetRequestError)?;

        let status = response.status();
        info!("Response status: {status}");
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            debug!("Response status was {} instead of 200 or 206", status.as_str());
            return Err(ErrorKind::ResponseStatusError(status.to_string()))?;
        }
        
        info!("Copying content to output file...");
        let bytes = response.bytes().await.chain_err(|| ErrorKind::GetRequestError)?;
        tokio::io::copy(&mut &*bytes, &mut output_file)
            .await.chain_err(|| ErrorKind::FileError("copying response to file".to_string()))?; 
    } 
    
    info!("Finished successfuly!");
    Ok(())
}

/// Helper macro to call download_video without the chunk_size parameter
///
/// # Example
/// ```
/// # #[macro_use] extern crate ytb_downloader;
/// # use ytb_downloader::*;
/// # use errors::*;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
///     .into_iter().next().unwrap();
/// const OUTPUT_FILE: &str = "download.m4a"; 
/// download_video!(&source, OUTPUT_FILE).await;
/// Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! download_video {
    ( $s:expr,$o:expr ) => {
        download_video($s,$o,None)
    };
}

