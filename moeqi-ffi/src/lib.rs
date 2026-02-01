use moeqi_core::types::{CodecConfig, Image, PixelFormat};
use moeqi_core::{format, Result};

#[repr(C)]
pub struct MoeqiBuf {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

fn vec_to_buf(mut v: Vec<u8>) -> MoeqiBuf {
    let out = MoeqiBuf {
        ptr: v.as_mut_ptr(),
        len: v.len(),
        cap: v.capacity(),
    };
    core::mem::forget(v);
    out
}

#[no_mangle]
pub extern "C" fn moeqi_free_buf(b: MoeqiBuf) {
    if b.ptr.is_null() || b.cap == 0 {
        return;
    }
    unsafe { drop(Vec::from_raw_parts(b.ptr, b.len, b.cap)) }
}

/// Encode raw pixels (Gray8/RGB8/RGBA8) into MOEQI1 container bytes.
/// cfg_json: UTF-8 JSON of CodecConfig, or null for default.
#[no_mangle]
pub extern "C" fn moeqi_encode(
    pixels: *const u8,
    pixels_len: usize,
    width: u32,
    height: u32,
    format_tag: u8,
    cfg_json: *const u8,
    cfg_json_len: usize,
) -> MoeqiBuf {
    let r = (|| -> Result<Vec<u8>> {
        let format = match format_tag {
            1 => PixelFormat::Gray8,
            3 => PixelFormat::Rgb8,
            4 => PixelFormat::Rgba8,
            _ => return Err(moeqi_core::MoeqiError::InvalidData("bad format_tag")),
        };

        let data = unsafe { core::slice::from_raw_parts(pixels, pixels_len) }.to_vec();
        let img = Image { width, height, format, data };

        let cfg = if cfg_json.is_null() || cfg_json_len == 0 {
            CodecConfig::default()
        } else {
            let s = core::str::from_utf8(unsafe { core::slice::from_raw_parts(cfg_json, cfg_json_len) })
                .map_err(|_| moeqi_core::MoeqiError::InvalidData("cfg_json not utf8"))?;
            serde_json::from_str::<CodecConfig>(s).map_err(|_| moeqi_core::MoeqiError::InvalidData("cfg_json parse"))?
        };

        format::binary::encode(&img, cfg)
    })();

    match r {
        Ok(v) => vec_to_buf(v),
        Err(_) => MoeqiBuf { ptr: core::ptr::null_mut(), len: 0, cap: 0 },
    }
}

#[cfg(feature = "wasm")]
mod wasm_api {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn encode_moeqi(pixels: &[u8], width: u32, height: u32, format_tag: u8, cfg_json: Option<String>) -> Vec<u8> {
        let cfg = cfg_json
            .and_then(|s| serde_json::from_str::<CodecConfig>(&s).ok())
            .unwrap_or_default();

        let format = match format_tag {
            1 => PixelFormat::Gray8,
            3 => PixelFormat::Rgb8,
            4 => PixelFormat::Rgba8,
            _ => PixelFormat::Rgba8,
        };

        let img = Image { width, height, format, data: pixels.to_vec() };
        format::binary::encode(&img, cfg).unwrap_or_default()
    }
}
