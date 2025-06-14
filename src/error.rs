use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidInput(String),
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    code: u16,
    timestamp: i64,
}

impl warp::reject::Reject for AppError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(app_err) = err.find::<AppError>() {
        match app_err {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Todo not found".to_string()),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::InternalError(msg) => {
                error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed".to_string())
    } else {
        error!("Unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };

    let json = warp::reply::json(&ErrorResponse {
        message,
        code: code.as_u16(),
        timestamp: chrono::Utc::now().timestamp(),
    });

    Ok(warp::reply::with_status(json, code))
}