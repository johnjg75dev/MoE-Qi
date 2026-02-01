use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormat {
    Gray8,
    Rgb8,
    Rgba8,
}
impl PixelFormat {
    pub fn channels(self) -> usize {
        match self {
            PixelFormat::Gray8 => 1,
            PixelFormat::Rgb8 => 3,
            PixelFormat::Rgba8 => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    /// Row-major, tightly packed.
    pub data: Vec<u8>,
}
impl Image {
    pub fn expected_len(&self) -> usize {
        self.width as usize * self.height as usize * self.format.channels()
    }
    pub fn validate(&self) -> bool {
        self.data.len() == self.expected_len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodecKind {
    PredictVarint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorTransform {
    None,
    /// Reversible integer transform (better residuals for RGB/RGBA)
    YCoCgR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodecConfig {
    pub codec: CodecKind,
    /// 0 = lossless (no quant). Otherwise signed uniform quant on residuals.
    pub quant_bits: u8,
    /// CRITICAL: keep predictor in reconstructed domain to avoid drift/artifacts.
    pub strict_recon: bool,
    pub color_transform: ColorTransform,
}

impl Default for CodecConfig {
    fn default() -> Self {
        Self {
            codec: CodecKind::PredictVarint,
            quant_bits: 0,
            strict_recon: true,
            color_transform: ColorTransform::YCoCgR,
        }
    }
}
