#![doc = include_str!("../README.md")]

pub use moeqi_core::{CodecConfig, CodecKind, ColorTransform, Image, MoeqiError, PixelFormat, Result};

/// Encode an [`Image`] into the `MOEQI1` binary container format.
pub fn encode(img: &Image, cfg: CodecConfig) -> Result<Vec<u8>> {
    moeqi_core::format::binary::encode(img, cfg)
}

/// Decode an `MOEQI1` binary container into an [`Image`] and the parsed [`CodecConfig`].
pub fn decode(bytes: &[u8]) -> Result<(Image, CodecConfig)> {
    moeqi_core::format::binary::decode(bytes)
}
