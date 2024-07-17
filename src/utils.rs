use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn hashmap_to_headers(
    hashmap: HashMap<String, String>,
) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    for (key, value) in hashmap {
        headers.insert(
            HeaderName::from_bytes(key.as_bytes())?,
            HeaderValue::from_str(&value)?,
        );
    }
    Ok(headers)
}
