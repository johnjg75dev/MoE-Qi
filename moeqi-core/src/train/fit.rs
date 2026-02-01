use crate::codec::{decode_payload, encode_payload};
use crate::error::Result;
use crate::train::eval;
use crate::types::{CodecConfig, Image};

/// Very simple “fit”: try quant_bits in a range and pick best score.
/// score = PSNR - lambda * bpp
pub fn fit_quant_bits(
    images: &[Image],
    mut cfg: CodecConfig,
    quant_candidates: &[u8],
    lambda_bpp: f64,
) -> Result<CodecConfig> {
    let mut best_cfg = cfg;
    let mut best_score = f64::NEG_INFINITY;

    for &qb in quant_candidates {
        cfg.quant_bits = qb;

        let mut psnr_acc = 0.0;
        let mut bpp_acc = 0.0;

        for img in images {
            let payload = encode_payload(img, cfg)?;
            let recon = decode_payload(&payload, img.width, img.height, img.format, cfg)?;
            psnr_acc += eval::psnr(img, &recon)?;
            bpp_acc += eval::bpp(payload.len(), img);
        }

        let n = images.len() as f64;
        let ps = psnr_acc / n;
        let bp = bpp_acc / n;
        let score = ps - lambda_bpp * bp;

        if score > best_score {
            best_score = score;
            best_cfg = cfg;
        }
    }

    Ok(best_cfg)
}
