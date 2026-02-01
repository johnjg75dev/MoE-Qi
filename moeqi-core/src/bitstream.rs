use crate::model::Model;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Codec {
    Varint,
    Huff,
}

pub struct Bitstream {
    pub w: u16,
    pub h: u16,
    pub qstep: u16,
    pub codec: Codec,

    // Borders
    pub first_row: Vec<u8>, // len = w
    pub first_col: Vec<u8>, // len = h

    // Residuals encoding
    pub residuals_count: u32, // (w-1)*(h-1)
    pub payload: Vec<u8>,

    // Huff table (if Huff)
    pub huff_symbols: Vec<i16>,
    pub huff_lengths: Vec<u8>,

    // Model
    pub model: Model,
}
