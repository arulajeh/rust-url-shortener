use serde::Serialize;

/// Model untuk response dari endpoint redirect
#[derive(Serialize)]
pub struct RedirectResponse {
    #[serde(rename = "originalUrl")]
    pub original_url: String,
}
