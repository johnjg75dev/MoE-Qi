use crate::error::MoeqiError;
use crate::bitstream::{Bitstream, Codec};
use crate::model::{Model, FEAT};

const MAGIC: &[u8; 8] = b"MOEQIBIN";
const VERSION: u8 = 2;

#[repr(u8)]
enum Quant { Fp32=0, Fp16=1, Int8=2 }

fn rd_u8(data: &[u8], o: &mut usize) -> Result<u8, MoeqiError> {
    if *o+1 > data.len() { return Err(MoeqiError::Format("eof")); }
    let v = data[*o]; *o += 1; Ok(v)
}
fn rd_u16(data: &[u8], o: &mut usize) -> Result<u16, MoeqiError> {
    if *o+2 > data.len() { return Err(MoeqiError::Format("eof")); }
    let v = u16::from_le_bytes([data[*o], data[*o+1]]); *o += 2; Ok(v)
}
fn rd_u32(data: &[u8], o: &mut usize) -> Result<u32, MoeqiError> {
    if *o+4 > data.len() { return Err(MoeqiError::Format("eof")); }
    let v = u32::from_le_bytes([data[*o], data[*o+1], data[*o+2], data[*o+3]]); *o += 4; Ok(v)
}
fn rd_f32(data: &[u8], o: &mut usize) -> Result<f32, MoeqiError> {
    if *o+4 > data.len() { return Err(MoeqiError::Format("eof")); }
    let v = f32::from_le_bytes([data[*o], data[*o+1], data[*o+2], data[*o+3]]); *o += 4; Ok(v)
}
fn rd_bytes(data: &[u8], o: &mut usize, n: usize) -> Result<Vec<u8>, MoeqiError> {
    if *o+n > data.len() { return Err(MoeqiError::Format("eof")); }
    let v = data[*o..*o+n].to_vec(); *o += n; Ok(v)
}
fn rd_f32_vec(data: &[u8], o: &mut usize, n: usize) -> Result<Vec<f32>, MoeqiError> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(rd_f32(data, o)?);
    }
    Ok(out)
}

pub fn parse_mqb(bytes: &[u8]) -> Result<Bitstream, MoeqiError> {
    if bytes.len() < 10 { return Err(MoeqiError::Format("too small")); }
    if &bytes[0..8] != MAGIC { return Err(MoeqiError::Format("bad magic")); }
    let mut o = 8usize;
    let ver = rd_u8(bytes, &mut o)?;
    if ver != VERSION { return Err(MoeqiError::Unsupported("version")); }

    let _flags = rd_u8(bytes, &mut o)?; // reserved (zlib container handled outside if you add it)
    let codec_id = rd_u8(bytes, &mut o)?;
    let codec = match codec_id {
        0 => Codec::Varint,
        1 => Codec::Huff,
        _ => return Err(MoeqiError::Unsupported("codec")),
    };

    // NEW v2: quant_wr, quant_we
    let _quant_wr = rd_u8(bytes, &mut o)?; // must be fp32 in this design
    let quant_we = rd_u8(bytes, &mut o)?;

    let w = rd_u16(bytes, &mut o)?;
    let h = rd_u16(bytes, &mut o)?;
    let qstep = rd_u16(bytes, &mut o)?;
    let e = rd_u16(bytes, &mut o)?;
    let residuals_count = rd_u32(bytes, &mut o)?;

    let first_row = rd_bytes(bytes, &mut o, w as usize)?;
    let first_col = rd_bytes(bytes, &mut o, h as usize)?;

    let payload_len = rd_u32(bytes, &mut o)? as usize;
    let payload = rd_bytes(bytes, &mut o, payload_len)?;

    let mut huff_symbols = Vec::new();
    let mut huff_lengths = Vec::new();
    if codec == Codec::Huff {
        let nsym = rd_u16(bytes, &mut o)? as usize;
        huff_symbols.reserve(nsym);
        huff_lengths.reserve(nsym);
        for _ in 0..nsym {
            // i16 sym, u8 len
            let sym = i16::from_le_bytes([rd_u8(bytes,&mut o)?, rd_u8(bytes,&mut o)?]);
            let ln = rd_u8(bytes, &mut o)?;
            huff_symbols.push(sym);
            huff_lengths.push(ln);
        }
    }

    let nvals = (e as usize) * FEAT;

    // Wr fp32 always
    let wr = rd_f32_vec(bytes, &mut o, nvals)?;

    // We quantized
    let we: Vec<f32> = match quant_we {
        x if x == Quant::Fp32 as u8 => rd_f32_vec(bytes, &mut o, nvals)?,
        x if x == Quant::Fp16 as u8 => {
            // fp16 -> f32
            let mut out = Vec::with_capacity(nvals);
            for _ in 0..nvals {
                let lo = rd_u8(bytes, &mut o)?;
                let hi = rd_u8(bytes, &mut o)?;
                let half = u16::from_le_bytes([lo, hi]);
                out.push(half_to_f32(half));
            }
            out
        }
        x if x == Quant::Int8 as u8 => {
            let scale = rd_f32(bytes, &mut o)?;
            let raw = rd_bytes(bytes, &mut o, nvals)?;
            raw.into_iter().map(|b| (b as i8 as f32) * scale).collect()
        }
        _ => return Err(MoeqiError::Unsupported("quant_we")),
    };

    Ok(Bitstream {
        w, h, qstep, codec,
        first_row, first_col,
        residuals_count,
        payload,
        huff_symbols, huff_lengths,
        model: Model { e, wr, we },
    })
}

// Minimal fp16->f32 converter (no deps)
fn half_to_f32(h: u16) -> f32 {
    // IEEE 754 half conversion (simple, not ultra-optimized)
    let sign = ((h >> 15) & 1) as u32;
    let exp  = ((h >> 10) & 0x1f) as u32;
    let frac = (h & 0x03ff) as u32;

    let f: u32 = if exp == 0 {
        if frac == 0 {
            sign << 31
        } else {
            // subnormal
            let mut e = -14i32;
            let mut m = frac;
            while (m & 0x0400) == 0 { m <<= 1; e -= 1; }
            m &= 0x03ff;
            let exp32 = (e + 127) as u32;
            (sign<<31) | (exp32<<23) | (m<<13)
        }
    } else if exp == 31 {
        // inf/nan
        (sign<<31) | (0xff<<23) | (frac<<13)
    } else {
        let exp32 = (exp as i32 - 15 + 127) as u32;
        (sign<<31) | (exp32<<23) | (frac<<13)
    };

    f32::from_bits(f)
}

// pack_mqb can be added later (mirror your format)
pub fn pack_mqb(_bs: &Bitstream) -> Result<Vec<u8>, MoeqiError> {
    Err(MoeqiError::Unsupported("packer not implemented yet"))
}
