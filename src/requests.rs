use super::errors::*;
use reqwest::header::HeaderValue;

// Credits to Rust Cookbook
// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
pub struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            Err("invalid buffer_size, give a value greater than zero.")?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
            )
        }
    }
}

pub fn create_request_body(vid: &str) -> String {
    format!(
        "{{
        \"context\": {{
            \"client\": {{
                \"hl\": \"en\",
                \"clientName\": \"WEB\",
                \"clientVersion\": \"2.20210721.00.00\",
                \"clientFormFactor\": \"UNKNOWN_FORM_FACTOR\",
                \"clientScreen\": \"WATCH\",
                \"mainAppWebInfo\": {{
                    \"graftUrl\": \"/watch?v={vid}\"
                }}
            }},
            \"user\": {{
                \"lockedSafetyMode\": false
            }},
            \"request\": {{
                \"useSsl\": true,
                \"internalExperimentFlags\": [],
                \"consistencyTokenJars\": []
            }}
        }},
        \"videoId\": \"{vid}\",
        \"playbackContext\": {{
            \"contentPlaybackContext\": {{
                \"vis\": 0,
                \"splay\": false,
                \"autoCaptionsDefaultOn\": false,
                \"autonavState\": \"STATE_NONE\",
                \"html5Preference\": \"HTML5_PREF_WANTS\",
                \"lactMilliseconds\": \"-1\"
            }}
        }},
        \"racyCheckOk\": false,
        \"contentCheckOk\": false
        }}"
    )
}
