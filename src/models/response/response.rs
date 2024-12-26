use serde::Serialize;

/// Struktur konsisten untuk semua response API
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub code: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Membuat response sukses
    pub fn success(message: &str, code: &str, data: Option<T>) -> Self {
        ApiResponse {
            message: message.to_string(),
            code: code.to_string(),
            data,
        }
    }

    /// Membuat response error
    pub fn error(message: &str, code: &str) -> Self {
        ApiResponse {
            message: message.to_string(),
            code: code.to_string(),
            data: None,
        }
    }
}
