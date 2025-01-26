use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use jsonwebtoken::errors::Error as JwtError;
use mongodb::error::Error as MongoError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] MongoError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::JwtError(err) => HttpResponse::Unauthorized().body(err.to_string()),
            AppError::DatabaseError(err) => HttpResponse::InternalServerError().body(err.to_string()),
            AppError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
            AppError::InternalServerError => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}