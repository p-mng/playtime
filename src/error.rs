use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no app with this name found: {0}")]
    AppNotFound(String),

    #[error("an app with this name already exists: {0}")]
    AppExists(String),

    #[error("config directory not found")]
    NoConfigDir,

    #[error("config directory exists but is not a directory")]
    InvalidConfigDir,

    #[error("config file exists but is not a regular file")]
    InvalidConfigFile,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("error deserializing TOML {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("error serializing TOML {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("time-related errror: {0}")]
    Span(#[from] jiff::Error),
}
