use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(String),
    Validation(HashMap<String, String>),
}

#[derive(Serialize)]
enum ErrorResponse {
    Error {
        error: String,
    },

    ValidationErrors {
        validation_errors: HashMap<String, String>,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, ErrorResponse::Error { error: msg }),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::Error { error: msg },
            ),
            AppError::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::ValidationErrors {
                    validation_errors: errors,
                },
            ),
        };
        (status, Json(message)).into_response()
    }
}
