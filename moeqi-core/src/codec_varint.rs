// crates/moeqi-core/src/codec_varint.rs
//
// Simple varint codec for i16 residuals using ZigZag encoding.
// - Encodes signed i16 -> unsigned varint (LEB128-style).
// - Decodes payload back into i16.
// Deterministic and portable across wasm/native.

#[inline]
fn zigzag_i32(x: i32) -> u32 {
    // Maps signed -> unsigned so small magnitudes become small numbers:
    //  0 -> 0
    // -1 -> 1
    //  1 -> 2
    // -2 -> 3
    //  2 -> 4
    ((x << 1) ^ (x >> 31)) as u32
}

#[inline]
fn unzigzag_u32(u: u32) -> i32 {
    // Inverse of zigzag
    ((u >> 1) as i32) ^ (-((u & 1) as i32))
}

#[inline]
fn push_varint_u32(mut u: u32, out: &mut Vec<u8>) {
    // LEB128 varint (7 bits per byte, MSB=continuation)
    while u >= 0x80 {
        out.push(((u as u8) & 0x7F) | 0x80);
        u >>= 7;
    }
    out.push(u as u8);
}

/// Encode a slice of i16 residuals into a varint payload.
///
/// Returns the encoded byte payload.
pub fn encode_varint_i16(vals: &[i16]) -> Vec<u8> {
    // Rough guess: many small residuals -> ~1 byte each
    let mut out = Vec::with_capacity(vals.len());
    for &v in vals {
        let u = zigzag_i32(v as i32);
        push_varint_u32(u, &mut out);
    }
    out
}

/// Decode varint payload into exactly `count` i16 values.
/// Errors if payload is truncated or malformed.
///
/// Notes:
/// - We decode u32, then unzigzag to i32, then clamp to i16 range.
/// - If your encoder guarantees i16 range, this is exact.
pub fn decode_varint_i16(data: &[u8], count: usize) -> Result<Vec<i16>, ()> {
    let mut out: Vec<i16> = Vec::with_capacity(count);
    let mut i = 0usize;

    while out.len() < count {
        if i >= data.len() {
            return Err(());
        }
        let mut shift = 0u32;
        let mut u: u32 = 0;

        loop {
            if i >= data.len() {
                return Err(());
            }
            let b = data[i];
            i += 1;

            u |= ((b & 0x7F) as u32) << shift;

            if (b & 0x80) == 0 {
                break;
            }
            shift += 7;
            if shift > 28 {
                // would overflow u32 or indicates corrupted stream
                return Err(());
            }
        }

        let v = unzigzag_u32(u);

        // If you want strict validation:
        // if v < i16::MIN as i32 || v > i16::MAX as i32 { return Err(()); }
        // Otherwise clamp:
        let v = v.clamp(i16::MIN as i32, i16::MAX as i32);

        out.push(v as i16);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_small_values() {
        let vals: Vec<i16> = vec![0, 1, -1, 2, -2, 127, -128, 300, -300, i16::MAX, i16::MIN];
        let enc = encode_varint_i16(&vals);
        let dec = decode_varint_i16(&enc, vals.len()).unwrap();
        assert_eq!(dec, vals);
    }

    #[test]
    fn decode_rejects_truncated() {
        let vals: Vec<i16> = vec![0, 1, -1, 2, -2, 127];
        let mut enc = encode_varint_i16(&vals);
        enc.pop(); // truncate
        assert!(decode_varint_i16(&enc, vals.len()).is_err());
    }
}
