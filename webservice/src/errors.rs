use actix_web::{cookie::ParseError, error, http::StatusCode, HttpResponse, Result, body::BoxBody};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt::{self, Display};

#[derive(Debug, Serialize)]
pub enum AppError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl AppError {
    fn error_response(&self) -> String {
        match self {
            AppError::DBError(e) => {
                println!("Database error occurred: {:?}", e);
                "Database error".into()
            },
            AppError::ActixError(e) => {
                println!("Server error occurred: {:?}", e);
                "Internel server error".into()
            }
            AppError::NotFound(e) => {
                println!("Not Found error occurred: {:?}", e);
                e.into()
            }
        }
    }
}

impl error::ResponseError for AppError {
    // 返回的状态码
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND
        }
    }

    // 返回的响应提
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<error::Error> for AppError {
    fn from(value: error::Error) -> Self {
        AppError::ActixError(value.to_string())
    }
}

impl From<SQLxError> for AppError {
    fn from(value: SQLxError) -> Self {
        AppError::DBError(value.to_string())
    }
}

