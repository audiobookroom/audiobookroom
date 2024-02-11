use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AudioAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl AudioAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AudioAppError::NotFound => StatusCode::NOT_FOUND,
            AudioAppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
