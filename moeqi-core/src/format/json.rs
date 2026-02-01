use crate::error::Result;
use crate::types::{CodecConfig, Image};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonBundle {
    pub image: Image,
    pub config: CodecConfig,
    /// Base64 optional later; for now raw bytes in JSON arrays is fine for debugging.
    pub encoded: Vec<u8>,
}

pub fn encode_bundle(img: &Image, cfg: CodecConfig) -> Result<String> {
    let encoded = crate::format::binary::encode(img, cfg)?;
    let bundle = JsonBundle {
        image: img.clone(),
        config: cfg,
        encoded,
    };
    Ok(serde_json::to_string_pretty(&bundle)?)
}

pub fn decode_bundle(s: &str) -> Result<JsonBundle> {
    Ok(serde_json::from_str(s)?)
}
