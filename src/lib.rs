mod chunker;
mod kv;
mod totter;

//const SEPARATOR: char = ':';

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IoError: {0:?}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type Record = std::collections::HashMap<String, String>;
pub type Records = Vec<Record>;

pub use totter::Tot;
