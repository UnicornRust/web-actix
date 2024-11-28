use serde::Serialize;
use actix_web::{error, HttpResponse};
use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum AppError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    error_message: String,
}

impl Error for AppError {}

impl AppError {
    fn error_response(&self) -> String {
        match self {
            AppError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            AppError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            AppError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                "Not found".into()
            }
        }
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> awc::http::StatusCode {
        match self {
            AppError::ActixError(_) => awc::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TeraError(_) => awc::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => awc::http::StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<awc::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .json(AppErrorResponse{error_message: self.error_response()})
    }
}


impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<error::Error> for AppError {
    fn from(value: error::Error) -> Self {
        AppError::ActixError(value.to_string())
    }
}
