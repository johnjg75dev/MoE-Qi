use crate::error::{MoeqiError, Result};

pub fn encode_u32_var(mut v: u32, out: &mut Vec<u8>) {
    while v >= 0x80 {
        out.push(((v as u8) & 0x7F) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

pub fn decode_u32_var(input: &[u8]) -> Result<(u32, usize)> {
    let mut shift = 0u32;
    let mut val = 0u32;

    for (i, &b) in input.iter().enumerate() {
        let chunk = (b & 0x7F) as u32;
        val |= chunk << shift;

        if (b & 0x80) == 0 {
            return Ok((val, i + 1));
        }
        shift += 7;
        if shift > 28 {
            return Err(MoeqiError::InvalidData("varint overflow"));
        }
    }

    Err(MoeqiError::Eof)
}
