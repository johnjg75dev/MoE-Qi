use crate::huff_canonical::{build_tree, Node};

pub fn decode_huff_i16(
    payload: &[u8],
    count: usize,
    symbols: &[i16],
    lengths: &[u8],
) -> Result<Vec<i16>, ()> {
    let root = build_tree(symbols, lengths)?;
    let mut out = Vec::with_capacity(count);

    let mut node: &Node = &root;

    for &byte in payload {
        let mut b = byte;
        // LSB-first bits (matches your JS encoder assumption)
        for _ in 0..8 {
            let bit = (b & 1) != 0;
            b >>= 1;

            node = if bit {
                node.right.as_deref().ok_or(())?
            } else {
                node.left.as_deref().ok_or(())?
            };

            if let Some(sym) = node.sym {
                out.push(sym);
                if out.len() == count {
                    return Ok(out);
                }
                node = &root;
            }
        }
    }
    Err(())
}
