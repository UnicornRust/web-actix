use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize)]
pub enum AppError {
    SomeError(String)
}


impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::SomeError(value)
    }
}

impl From<JsValue> for AppError {
    fn from(value: JsValue) -> Self {
        AppError::SomeError(value.as_string().unwrap())
    }
}

