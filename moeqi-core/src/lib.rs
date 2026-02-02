#![doc = include_str!("../README.md")]

pub mod codec;
pub mod error;
pub mod format;
pub mod train;
pub mod types;

pub use error::{MoeqiError, Result};
pub use types::{CodecConfig, CodecKind, ColorTransform, Image, PixelFormat};
