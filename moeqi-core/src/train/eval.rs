use crate::error::{MoeqiError, Result};
use crate::types::{Image, PixelFormat};

pub fn mse(a: &Image, b: &Image) -> Result<f64> {
    if a.width != b.width || a.height != b.height || a.format != b.format {
        return Err(MoeqiError::InvalidData("image mismatch"));
    }
    let mut acc = 0f64;
    for (&x, &y) in a.data.iter().zip(b.data.iter()) {
        let d = x as f64 - y as f64;
        acc += d * d;
    }
    Ok(acc / (a.data.len() as f64))
}

pub fn psnr(a: &Image, b: &Image) -> Result<f64> {
    let m = mse(a, b)?;
    if m == 0.0 {
        return Ok(99.0);
    }
    let max = 255.0;
    Ok(20.0 * (max / m.sqrt()).log10())
}

pub fn bpp(encoded_bytes: usize, img: &Image) -> f64 {
    let pixels = (img.width as f64) * (img.height as f64);
    (encoded_bytes as f64 * 8.0) / pixels
}

pub fn is_color(fmt: PixelFormat) -> bool {
    matches!(fmt, PixelFormat::Rgb8 | PixelFormat::Rgba8)
}
