#![doc = include_str!("../README.md")]

use moeqi_core::types::{CodecConfig, Image, PixelFormat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Decoded {
    w: u32,
    h: u32,
    data: Vec<u8>,
}

/*#[wasm_bindgen]
impl Decoded {
    #[wasm_bindgen(getter)]
    pub fn w(&self) -> u32 { self.w }
    #[wasm_bindgen(getter)]
    pub fn h(&self) -> u32 { self.h }
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> { self.data.clone() } // simplest; can optimize with Uint8Array views
}*/

/*#[wasm_bindgen]
pub fn parse_mqb(bytes: &[u8]) -> Result<(Image, crate::types::CodecConfig)> {
    // Old name, new container.
    format::binary::decode(bytes)
}*/

/*pub fn decode_luma(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let (img, _cfg) = format::binary::decode(bytes)?;
    let (width, height) = (img.width, img.height);

    fn rgb_to_luma(r: u8, g: u8, b: u8) -> u8 {
        let (r, g, b) = (r as u16, g as u16, b as u16);
        ((77 * r + 150 * g + 29 * b + 128) >> 8) as u8
    }

    let luma = match img.format {
        PixelFormat::Gray8 => img.data,
        PixelFormat::Rgb8 => img
            .data
            .chunks_exact(3)
            .map(|p| rgb_to_luma(p[0], p[1], p[2]))
            .collect(),
        PixelFormat::Rgba8 => img
            .data
            .chunks_exact(4)
            .map(|p| rgb_to_luma(p[0], p[1], p[2]))
            .collect(),
    };

    if (width as usize) * (height as usize) != luma.len() {
        return Err(MoeqiError::InvalidData("luma length mismatch"));
    }

    Ok((width, height, luma))
}*/

/*#[wasm_bindgen]
fn to_luma_bytes(img: &Image) -> Vec<u8> {
    match img.format {
        PixelFormat::Gray8 => img.data.clone(),
        PixelFormat::Rgb8 => img.data.chunks_exact(3).map(|p| {
            // integer luma approx: (0.299,0.587,0.114)
            let r = p[0] as u16;
            let g = p[1] as u16;
            let b = p[2] as u16;
            ((77*r + 150*g + 29*b + 128) >> 8) as u8
        }).collect(),
        PixelFormat::Rgba8 => img.data.chunks_exact(4).map(|p| {
            let r = p[0] as u16;
            let g = p[1] as u16;
            let b = p[2] as u16;
            ((77*r + 150*g + 29*b + 128) >> 8) as u8
        }).collect(),
    }
}*/
