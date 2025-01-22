use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Path not found: {0}")]
    PathNotFound(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Upload error: {0}")]
    UploadError(String),
}

pub type AppResult<T> = Result<T, AppError>;
