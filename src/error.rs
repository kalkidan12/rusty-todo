use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Todo not found")]
    NotFound,

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),
}
