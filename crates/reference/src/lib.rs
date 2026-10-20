//! Public no_std generic reference API. The independent benchmark oracle lives
//! in scripts/reference.py and does not call this implementation.
#![no_std]
pub use kernel_core::{generic, requantize, Params};
