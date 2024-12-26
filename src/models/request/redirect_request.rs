use serde::Deserialize;

/// Model untuk request mendapatkan URL asli
#[derive(Deserialize)]
pub struct RedirectRequest {
    pub short: String,
}
