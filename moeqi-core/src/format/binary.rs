use crate::codec::{decode_payload, encode_payload};
use crate::error::{MoeqiError, Result};
use crate::types::{CodecConfig, Image, PixelFormat};

const MAGIC: &[u8; 6] = b"MOEQI1";

pub fn encode(img: &Image, cfg: CodecConfig) -> Result<Vec<u8>> {
    let payload = encode_payload(img, cfg)?;

    let mut out = Vec::with_capacity(32 + payload.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&img.width.to_le_bytes());
    out.extend_from_slice(&img.height.to_le_bytes());
    out.push(match img.format {
        PixelFormat::Gray8 => 1,
        PixelFormat::Rgb8 => 3,
        PixelFormat::Rgba8 => 4,
    });
    out.push(cfg.quant_bits);
    out.push(if cfg.strict_recon { 1 } else { 0 });
    out.push(match cfg.color_transform {
        crate::types::ColorTransform::None => 0,
        crate::types::ColorTransform::YCoCgR => 1,
    });

    // payload length u32
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&payload);
    Ok(out)
}

pub fn decode(bytes: &[u8]) -> Result<(Image, CodecConfig)> {
    if bytes.len() < 6 + 4 + 4 + 1 + 1 + 1 + 1 + 4 {
        return Err(MoeqiError::InvalidData("too small"));
    }
    if &bytes[0..6] != MAGIC {
        return Err(MoeqiError::InvalidData("bad magic"));
    }
    let mut o = 6usize;

    let width = u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    o += 4;
    let height = u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    o += 4;

    let fmt = match bytes[o] {
        1 => PixelFormat::Gray8,
        3 => PixelFormat::Rgb8,
        4 => PixelFormat::Rgba8,
        _ => return Err(MoeqiError::InvalidData("bad pixel format")),
    };
    o += 1;

    let quant_bits = bytes[o];
    o += 1;
    let strict_recon = bytes[o] != 0;
    o += 1;
    let color_transform = match bytes[o] {
        0 => crate::types::ColorTransform::None,
        1 => crate::types::ColorTransform::YCoCgR,
        _ => return Err(MoeqiError::InvalidData("bad color transform")),
    };
    o += 1;

    let pay_len = u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap()) as usize;
    o += 4;
    if bytes.len() < o + pay_len {
        return Err(MoeqiError::Eof);
    }
    let payload = &bytes[o..o + pay_len];

    let cfg = CodecConfig {
        codec: crate::types::CodecKind::PredictVarint,
        quant_bits,
        strict_recon,
        color_transform,
    };

    let img = decode_payload(payload, width, height, fmt, cfg)?;
    Ok((img, cfg))
}
