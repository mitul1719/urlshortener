use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    pub validity: u64, // validity in seconds
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
