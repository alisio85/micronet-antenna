#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Debug, Error)]
/// Error type used by the `micronet-antenna` std wrapper.
///
/// The core crate (`micronet-antenna-core`) is transport-agnostic and does not
/// define I/O errors. This error type exists to support std-only adapters such as
/// UDP transport and serialization.
pub enum Error {
    #[error("serialization error: {0}")]
    /// Serialization or deserialization failed.
    Serialization(String),

    #[error("io error: {0}")]
    /// I/O failure from the underlying operating system.
    Io(#[from] std::io::Error),
}

#[cfg(not(feature = "std"))]
#[derive(Debug)]
/// Minimal error type for non-std builds.
///
/// The `micronet-antenna` crate is intended to be std-only; for `no_std` usage
/// depend on `micronet-antenna-core` instead.
pub enum Error {
    Serialization,
}

/// Convenience result alias for std wrapper operations.
pub type Result<T> = core::result::Result<T, Error>;
