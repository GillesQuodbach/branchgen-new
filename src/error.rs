use thiserror::Error;
#[derive(Error, Debug)]
enum AppError {
    #[error("Empty field : {0}")]
    EmptyField(String),
    #[error("Config error : {0}")]
    Config(String),
    #[error("Git error : {0}")]
    Git(String),
    #[error("I/O error : {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error : {0}")]
    Toml(#[from] toml::de::Error),
    #[error("JSON parse error : {0}")]
    Json(#[from] serde_json::Error),
}