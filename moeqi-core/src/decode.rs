use crate::{MoeqiError};
use crate::bitstream::{Bitstream, Codec};
use crate::model::{FEAT, router_argmax, dot7};

use crate::codec_varint::decode_varint_i16;
use crate::codec_huff::decode_huff_i16;
use crate::codec_varint::encode_varint_i16;

#[inline]
fn clamp_u8(x: i32) -> u8 {
    if x < 0 { 0 } else if x > 255 { 255 } else { x as u8 }
}

#[inline]
fn feat_at(x: usize, y: usize, w: usize, luma: &[u8]) -> [f32; FEAT] {
    let idx = y*w + x;
    let cur = luma[idx] as i32;
    let l  = if x>0 { luma[idx-1] as i32 } else { cur };
    let u  = if y>0 { luma[idx-w] as i32 } else { cur };
    let ul = if x>0 && y>0 { luma[idx-w-1] as i32 } else { cur };

    let lf = (l as f32) / 255.0;
    let uf = (u as f32) / 255.0;
    let ulf = (ul as f32) / 255.0;

    [
        1.0,
        lf,
        uf,
        ulf,
        lf - uf,
        lf - ulf,
        uf - ulf,
    ]
}

pub fn decode_luma(bs: &Bitstream) -> Result<Vec<u8>, MoeqiError> {
    let w = bs.w as usize;
    let h = bs.h as usize;

    if bs.first_row.len() != w { return Err(MoeqiError::Format("first_row length")); }
    if bs.first_col.len() != h { return Err(MoeqiError::Format("first_col length")); }

    let expected = (w.saturating_sub(1) * h.saturating_sub(1)) as u32;
    if bs.residuals_count != expected {
        return Err(MoeqiError::Format("residuals_count mismatch"));
    }

    // Decode residuals to i16 qi
    let qi: Vec<i16> = match bs.codec {
        Codec::Varint => decode_varint_i16(&bs.payload, bs.residuals_count as usize)
            .map_err(|_| MoeqiError::Decode("varint"))?,
        Codec::Huff => decode_huff_i16(
            &bs.payload,
            bs.residuals_count as usize,
            &bs.huff_symbols,
            &bs.huff_lengths,
        ).map_err(|_| MoeqiError::Decode("huff"))?,
    };

    let mut recon = vec![0u8; w*h];

    // seed borders
    recon[0..w].copy_from_slice(&bs.first_row);
    for y in 0..h {
        recon[y*w] = bs.first_col[y];
    }

    let qstep = bs.qstep as i32;
    let mut ri = 0usize;

    for y in 1..h {
        for x in 1..w {
            let f = feat_at(x, y, w, &recon);
            let k = router_argmax(&bs.model, &f);

            let mu = dot7(bs.model.we_row(k), &f);
            // explicit rounding point (important for consistency)
            let pred = (mu * 255.0).round() as i32;

            let q = qi[ri] as i32;
            ri += 1;
            recon[y*w + x] = clamp_u8(pred + q*qstep);
        }
    }

    Ok(recon)
}
