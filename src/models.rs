use serde::{Deserialize, Serialize};


fn default_validity() -> u64 {
    // Set default validity to 24 hours (86400 seconds)
    24 * 60 * 60
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    #[serde(default = "default_validity")]
    pub validity: u64, // validity in seconds
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
    pub validity: u64
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
