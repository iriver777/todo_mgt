use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> where T: Serialize {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize)]
pub struct PaginatedResponse<T> where T: Serialize {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> where T: Serialize {
    pub fn new(items: Vec<T>, total: i64, page: u32, page_size: u32) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            ((total as f64) / (page_size as f64)).ceil() as u32
        };
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}
