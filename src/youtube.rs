use super::errors::*;
use crate::errors;
/// Functions related to requests to Youtube
use json::JsonValue;
use regex::Regex;
use std::str::FromStr;

/// Youtube endpoint for the POST request that returns available sources
pub static YOUTUBE_ENDPOINT: &str =
    "https://www.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Download source
pub struct DownloadSource {
    /// URL of video
    pub video_url: String,
    /// Mime Type <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types>
    pub mime_type: String,
    /// Quality of video
    pub quality: String,
    /// Content length
    pub content_length: u64,
    /// Width dimension of video
    pub width: Option<u32>,
    /// Length dimension of video
    pub height: Option<u32>,
    /// Frames per second
    pub fps: Option<u32>,
    /// Quality label present in audio formats
    pub quality_label: Option<String>,
    /// Bitrate
    pub bitrate: Option<u32>,
    /// Average bitrate
    pub avg_bitrate: u32,
    /// Audio quality present in audio formats
    pub audio_quality: Option<String>,
    /// Audio sample rate present in audio formats
    pub audio_sample_rate: Option<u32>,
    /// Audio channels present in audio formats
    pub audio_channels: Option<u8>,
}

/// Parses a Youtube video URL and returns the video ID as a String
///
/// ## Example
///
/// ```
/// # use ytb_downloader::youtube::youtube::parse_video_id;
/// let video_id = parse_video_id("https://www.youtube.com/watch?v=some_video").unwrap();
/// ```
pub fn parse_video_id(video_url: &str) -> Result<String> {
    let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
    let caps = url_regex
        .captures(video_url)
        .chain_err(|| ErrorKind::InvalidYoutubeLink)?;

    let video_id = caps
        .get(1)
        .chain_err(|| ErrorKind::InvalidYoutubeLink)?
        .as_str()
        .to_string();

    if video_id.is_empty() {
        return Err(ErrorKind::VideoIdEmpty)?;
    }

    Ok(video_id)
}

fn get_json_property(v: &JsonValue, name: &str) -> Result<String> {
    let prop = &v[name];
    if !prop.is_null() {
        return Ok(prop.to_string());
    };
    Err(ErrorKind::JsonPropertyError(name.to_string()))?
}

fn get_optional_json_property(v: &JsonValue, name: &str) -> Option<String> {
    let prop = &v[name];
    if !prop.is_null() {
        return Some(prop.to_string());
    };
    None
}

impl TryFrom<JsonValue> for DownloadSource {
    type Error = errors::Error;

    fn try_from(v: JsonValue) -> Result<Self> {
        let video_url = get_json_property(&v, "url")
            .chain_err(|| ErrorKind::JsonPropertyError("url".to_string()))?;

        let mime_type = get_json_property(&v, "mimeType")
            .chain_err(|| ErrorKind::JsonPropertyError("mimeType".to_string()))?;

        let quality = get_json_property(&v, "quality")
            .chain_err(|| ErrorKind::JsonPropertyError("quality".to_string()))?;

        let content_length = u64::from_str(
            get_json_property(&v, "contentLength")
                .chain_err(|| ErrorKind::JsonPropertyError("contentLength".to_string()))?
                .as_str(),
        )
        .chain_err(|| ErrorKind::JsonPropertyError("contentLength".to_string()))?;

        let width = match get_optional_json_property(&v, "width") {
            Some(s) => Some(
                u32::from_str(&s)
                    .chain_err(|| ErrorKind::JsonPropertyError("width".to_string()))?,
            ),
            None => None,
        };

        let height = match get_optional_json_property(&v, "height") {
            Some(s) => Some(
                u32::from_str(&s)
                    .chain_err(|| ErrorKind::JsonPropertyError("height".to_string()))?,
            ),
            None => None,
        };

        let fps = match get_optional_json_property(&v, "fps") {
            Some(s) => Some(
                u32::from_str(&s).chain_err(|| ErrorKind::JsonPropertyError("fps".to_string()))?,
            ),
            None => None,
        };

        let quality_label = get_optional_json_property(&v, "qualityLabel");

        let bitrate = match get_optional_json_property(&v, "bitrate") {
            Some(s) => Some(
                u32::from_str(&s)
                    .chain_err(|| ErrorKind::JsonPropertyError("bitrate".to_string()))?,
            ),
            None => None,
        };

        let avg_bitrate = u32::from_str(
            get_json_property(&v, "averageBitrate")
                .chain_err(|| ErrorKind::JsonPropertyError("averageBitrate".to_string()))?
                .as_str(),
        )
        .chain_err(|| ErrorKind::JsonPropertyError("averageBitrate".to_string()))?;

        let audio_quality = get_optional_json_property(&v, "audioQuality");

        let audio_sample_rate = match get_optional_json_property(&v, "audioSampleRate") {
            Some(s) => Some(
                u32::from_str(&s)
                    .chain_err(|| ErrorKind::JsonPropertyError("audioSampleRate".to_string()))?,
            ),
            None => None,
        };

        let audio_channels = match get_optional_json_property(&v, "audioChannels") {
            Some(s) => Some(
                u8::from_str(&s)
                    .chain_err(|| ErrorKind::JsonPropertyError("audioChannels".to_string()))?,
            ),
            None => None,
        };

        Ok(DownloadSource {
            video_url,
            mime_type,
            quality,
            content_length,
            width,
            height,
            fps,
            quality_label,
            bitrate,
            avg_bitrate,
            audio_quality,
            audio_sample_rate,
            audio_channels,
        })
    }
}

#[cfg(test)]
mod youtube_tests {
    use super::*;

    #[test]
    fn video_id_is_parsed() {
        assert_eq!(
            "pqhfyrW_BEA".to_string(),
            parse_video_id("https://www.youtube.com/watch?v=pqhfyrW_BEA").unwrap()
        );
    }

    #[test]
    fn invalid_video_id_throws() {
        let result = parse_video_id("https://www.youtube.com/watch?v=");
        assert!(result.is_err());

        let result = parse_video_id("jfejfielfilea");
        assert!(result.is_err());
    }
}
