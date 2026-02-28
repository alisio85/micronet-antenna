#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Debug, Error)]
pub enum Error {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub enum Error {
    Serialization,
}

pub type Result<T> = core::result::Result<T, Error>;
