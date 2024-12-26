use serde::Deserialize;

/// Model untuk request pembuatan URL pendek
#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}
