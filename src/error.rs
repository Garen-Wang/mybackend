use actix_multipart::MultipartError;
use actix_web::{http::StatusCode, HttpResponse, error::BlockingError};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(Value), // 401

    #[error("Forbidden: {0}")]
    Forbidden(Value), // 403

    #[error("Not Found: {0}")]
    NotFound(Value), // 404

    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(Value), // 422

    #[error("Internal Server Error")]
    InternalServerError,
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            AppError::Unauthorized(val) => HttpResponse::Unauthorized().json(val),
            AppError::Forbidden(val) => HttpResponse::Forbidden().json(val),
            AppError::NotFound(val) => HttpResponse::NotFound().json(val),
            AppError::UnprocessableEntity(val) => HttpResponse::UnprocessableEntity().json(val),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("internal server error")
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_e: bcrypt::BcryptError) -> Self {
        AppError::InternalServerError
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => AppError::NotFound(json!({
                "error": "requested record not found"
            })),
            _ => AppError::InternalServerError,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::Unauthorized(json!({
                "error": "invalid token"
            })),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
                "error": "invalid issuer"
            })),
            _ => AppError::Unauthorized(json!({
                "error": "a issue was found with token provided"
            })),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(_: r2d2::Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<MultipartError> for AppError {
    fn from(_: MultipartError) -> Self {
        AppError::InternalServerError
    }
}

impl From<BlockingError> for AppError {
    fn from(_: BlockingError) -> Self {
        AppError::InternalServerError
    }
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        AppError::InternalServerError
    }
}