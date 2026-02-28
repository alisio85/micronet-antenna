//! `micronet-antenna` is the **std-enabled wrapper** around `micronet-antenna-core`.
//!
//! - `micronet-antenna-core` contains the deterministic runtime/state and message types
//!   (designed to be usable in `no_std` + `alloc` environments).
//! - `micronet-antenna` adds std-only ergonomics such as network transport.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
compile_error!("micronet-antenna requires the `std` feature. For kernel/no_std usage, depend on `micronet-antenna-core`.");

pub use micronet_antenna_core::*;

mod error;

pub use error::{Error, Result};

pub mod transport;

pub use transport::Transport;
