//! SigmaOS-tee — Stream tee model.
//!
//! Models reading from standard input and writing to both standard output and files.
//! Deterministic and std-only.

pub mod tee;
pub mod options;

pub use tee::{Tee, TeeOutput};
pub use options::{TeeOptions, AppendMode};
