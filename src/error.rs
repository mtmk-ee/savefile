use std::path::PathBuf;

type SqliteError = rusqlite::Error;
type IoError = std::io::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] SqliteError),
    #[error("io error: {0}")]
    Io(#[from] IoError),
    #[error("profile error: {0}")]
    ProfileError(#[from] ProfileError),
    #[error("backup error: {0}")]
    BackupError(#[from] BackupError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProfileError {
    #[error("invalid profile format: {0}")]
    InvalidFormat(PathBuf),
    #[error("invalid profile format: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("no profile with the given name: {0}")]
    NoSuchProfile(PathBuf),
    #[error("profile already exists")]
    AlreadyExists,
    #[error("invalid base directory")]
    InvalidBase,
}

#[derive(thiserror::Error, Debug)]
pub enum BackupError {
    #[error("backups database is empty")]
    BackupsEmpty,
}
