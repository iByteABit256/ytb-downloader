pub mod youtube {
    use json::JsonValue;
    use crate::errors;
    use super::super::errors::*;
    use std::str::FromStr;

    pub static YOUTUBE_ENDPOINT: &str = "https://www.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct DownloadSource {
        pub video_url: String,
        pub mime_type: String,
        pub quality: String,
        pub content_length: u64,
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub fps: Option<u32>,
        pub quality_label: Option<String>,
        pub bitrate: Option<u32>,
        pub avg_bitrate: u32,
        pub audio_quality: Option<String>,
        pub audio_sample_rate: Option<u32>,
        pub audio_channels: Option<u8>,
    }

    fn get_json_property(v: &JsonValue, name: &str) -> Result<String> {
        let prop = &v[name];
        if !prop.is_null() {
            return Ok(prop.to_string())
        };
        bail!(format!("Json property '{}' required but was not found", name));
    }

    fn get_optional_json_property(v: &JsonValue, name: &str) -> Option<String>{
        let prop = &v[name];
        if !prop.is_null() {
            return Some(prop.to_string())
        };
        None
    }

    impl TryFrom<JsonValue> for DownloadSource {
        type Error = errors::Error;

        fn try_from(v: JsonValue) -> Result<Self> {
            let video_url = get_json_property(&v, "url").chain_err(|| "Url not found in adaptive format")?; 
            let mime_type = get_json_property(&v, "mimeType").chain_err(|| "Mime type not found in adaptive format")?;
            let quality = get_json_property(&v, "quality").chain_err(|| "Quality not found in adaptive format")?;
            let content_length = u64::from_str(get_json_property(&v, "contentLength").chain_err(|| "Content length not found in adaptive format")?.as_str())
                .chain_err(|| "Content length in adaptive format was not a valid number")?;
            let width = match get_optional_json_property(&v, "width") {
                Some(s) => Some(u32::from_str(&s).chain_err(|| "Width found in adaptive format but was incorrect format")?),
                None => None,
            };
            let height = match get_optional_json_property(&v, "height") {
                Some(s) => Some(u32::from_str(&s).chain_err(|| "Height found in adaptive format but was incorrect format")?),
                None => None,
            };
            let fps = match get_optional_json_property(&v, "fps") {
                Some(s) => Some(u32::from_str(&s).chain_err(|| "FPS found in adaptive format but was incorrect format")?),
                None => None,
            };
            let quality_label = get_optional_json_property(&v, "qualityLabel");
            let bitrate = match get_optional_json_property(&v, "bitrate") {
                Some(s) => Some(u32::from_str(&s).chain_err(|| "Bitrate found in adaptive format but was incorrect format")?),
                None => None,
            };
            let avg_bitrate = u32::from_str(get_json_property(&v, "averageBitrate").chain_err(|| "Average bitrate not found in adaptive format")?.as_str())
                .chain_err(|| "Average bitrate in adaptive format was not a valid number")?;
            let audio_quality = get_optional_json_property(&v, "audioQuality");
            let audio_sample_rate = match get_optional_json_property(&v, "audioSampleRate") {
                Some(s) => Some(u32::from_str(&s).chain_err(|| "Audio sample rate found in adaptive format but was incorrect format")?),
                None => None,
            };
            let audio_channels = match get_optional_json_property(&v, "audioChannels") {
                Some(s) => Some(u8::from_str(&s).chain_err(|| "Audio channels found in adaptive format but was incorrect format")?),
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
}

