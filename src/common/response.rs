use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> where T: Serialize {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
} 

impl<T> ApiResponse<T> where T: Serialize {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
