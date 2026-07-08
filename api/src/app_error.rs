use std::error::Error;

#[derive(Debug)]
pub enum AppError{
    Other(anyhow::Error),
    Database(sqlx::Error),

}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Other(err) => write!(f, "AppError: {}", err),
            AppError::Database(err) => write!(f, "AppError: {}", err),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(err)
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl Error for AppError {}
