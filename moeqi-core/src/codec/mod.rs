pub mod quant;
pub mod varint;

use crate::error::{MoeqiError, Result};
use crate::types::{CodecConfig, ColorTransform, Image, PixelFormat};
use quant::SignedUniformQuant;

#[inline]
fn zigzag_i16(v: i16) -> u16 {
    let x = v as i32;
    ((x << 1) ^ (x >> 15)) as u16
}
#[inline]
fn unzigzag_u16(v: u16) -> i16 {
    let x = v as i32;
    ((x >> 1) ^ (-(x & 1))) as i16
}

/// Encode image pixels to payload bytes (no container header).
pub fn encode_payload(img: &Image, cfg: CodecConfig) -> Result<Vec<u8>> {
    if !img.validate() {
        return Err(MoeqiError::InvalidData("image data length mismatch"));
    }

    let mut buf = img.data.clone();

    if cfg.color_transform == ColorTransform::YCoCgR {
        match img.format {
            PixelFormat::Rgb8 => rgb_to_ycocg(&mut buf, false),
            PixelFormat::Rgba8 => rgb_to_ycocg(&mut buf, true),
            PixelFormat::Gray8 => {}
        }
    }

    let q = if cfg.quant_bits == 0 {
        None
    } else {
        Some(SignedUniformQuant::new(cfg.quant_bits))
    };

    let ch = img.format.channels();
    let w = img.width as usize;
    let h = img.height as usize;

    let mut out = Vec::with_capacity(buf.len() / 2);

    for y in 0..h {
        for c in 0..ch {
            let mut prev: i16 = 0;
            for x in 0..w {
                let idx = (y * w + x) * ch + c;
                let cur = buf[idx] as i16;

                let mut res = cur - prev;
                if let Some(q) = &q {
                    res = q.quantize(res);
                }

                varint::encode_u32_var(zigzag_i16(res) as u32, &mut out);

                // 👇 THIS is the anti-artifact rule:
                // update predictor using reconstructed value (same as decoder).
                if cfg.strict_recon {
                    let recon = (prev + res).clamp(0, 255);
                    prev = recon;
                } else {
                    prev = cur;
                }
            }
        }
    }

    Ok(out)
}

/// Decode payload to Image (no container header).
pub fn decode_payload(
    payload: &[u8],
    width: u32,
    height: u32,
    format: PixelFormat,
    cfg: CodecConfig,
) -> Result<Image> {
    let q = if cfg.quant_bits == 0 {
        None
    } else {
        Some(SignedUniformQuant::new(cfg.quant_bits))
    };

    let ch = format.channels();
    let w = width as usize;
    let h = height as usize;

    let mut data = vec![0u8; w * h * ch];
    let mut i = 0usize;

    for y in 0..h {
        for c in 0..ch {
            let mut prev: i16 = 0;
            for x in 0..w {
                let (zz, used) = varint::decode_u32_var(&payload[i..])?;
                i += used;

                let mut res = unzigzag_u16(zz as u16);
                if let Some(q) = &q {
                    res = q.dequantize(res);
                }

                let cur = (prev + res).clamp(0, 255);
                let idx = (y * w + x) * ch + c;
                data[idx] = cur as u8;
                prev = cur;
            }
        }
    }

    if cfg.color_transform == ColorTransform::YCoCgR {
        match format {
            PixelFormat::Rgb8 => ycocg_to_rgb(&mut data, false),
            PixelFormat::Rgba8 => ycocg_to_rgb(&mut data, true),
            PixelFormat::Gray8 => {}
        }
    }

    Ok(Image { width, height, format, data })
}

// --- Reversible YCoCg-R in 8-bit lanes (with i16 math) ---

fn rgb_to_ycocg(buf: &mut [u8], has_alpha: bool) {
    let stride = if has_alpha { 4 } else { 3 };
    for px in buf.chunks_exact_mut(stride) {
        let r = px[0] as i16;
        let g = px[1] as i16;
        let b = px[2] as i16;

        let co = r - b;
        let t = b + (co >> 1);
        let cg = g - t;
        let y = t + (cg >> 1);

        px[0] = (y & 0xFF) as u8;
        px[1] = (co & 0xFF) as u8;
        px[2] = (cg & 0xFF) as u8;
    }
}

fn ycocg_to_rgb(buf: &mut [u8], has_alpha: bool) {
    let stride = if has_alpha { 4 } else { 3 };
    for px in buf.chunks_exact_mut(stride) {
        let y = px[0] as i16;
        let co = px[1] as i16;
        let cg = px[2] as i16;

        let t = y - (cg >> 1);
        let g = cg + t;
        let b = t - (co >> 1);
        let r = b + co;

        px[0] = r.clamp(0, 255) as u8;
        px[1] = g.clamp(0, 255) as u8;
        px[2] = b.clamp(0, 255) as u8;
    }
}
