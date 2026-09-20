use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError{
    #[error("{0}")]
    Other(String),    
    #[error("An DatabaseError accured: {0}")]
    Database(String),
    #[error("Wrong Password")]
    WrongPassword,
    #[error("internal server error: {0}")]
    ServerFnError(#[from] ServerFnError),
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(err.to_string())
    }
}

impl AsStatusCode for AppError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            AppError::Other(_)=> StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::WrongPassword=>StatusCode::UNAUTHORIZED,
            AppError::ServerFnError(e) => e.as_status_code(),
        }
    }
}