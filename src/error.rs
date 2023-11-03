use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = std::result::Result<T, WebError>;

#[derive(thiserror::Error, Debug)]
pub enum WebError {
    #[error("a custom error occured: {1}")]
    #[allow(dead_code)]
    CustomError(StatusCode, String),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("a reqwest error occured: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("a ical parser error occured: {0}")]
    IcalParserError(#[from] ical::parser::ParserError),
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::CustomError(status_code, message) => (
                status_code,
                Json(json!({
                    "message": message,
                })),
            )
                .into_response(),
            Self::ReqwestError(_) | Self::IcalParserError(_) | Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Something went wrong"})),
            )
                .into_response(),
        }
    }
}
