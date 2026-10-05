use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UniversalResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub timestamp: String,
    pub status_code: u16,
}

impl<T> UniversalResponse<T> {
    pub fn new(
        status: String,
        status_code: u16,
        data: T,
        timestamp: String,
    ) -> UniversalResponse<T> {
        Self {
            status,
            status_code,
            timestamp,
            data: Some(data),
        }
    }

    pub fn success(data: T) -> UniversalResponse<T> {
        UniversalResponse {
            status: "00".to_string(),
            data: Some(data),
            timestamp: chrono::Local::now().to_rfc3339(),
            status_code: 200,
        }
    }
}
