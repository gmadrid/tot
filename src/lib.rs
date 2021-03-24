mod chunker;
mod kv;
mod totter;

const SEPARATOR: char = ':';

#[derive(Debug, thiserror::Error)]
pub enum Error {}

type Result<T> = std::result::Result<T, Error>;

pub use totter::Tot;
