mod error;
mod storage;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
