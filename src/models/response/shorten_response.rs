use serde::Serialize;

/// Model untuk response dari endpoint shorten
#[derive(Serialize)]
pub struct ShortenResponse {
    pub short: String,
}
