
use serde::{Deserialize, Serialize};

#[cfg(feature = "json-array")]
pub mod array;

#[cfg(feature = "json-hash")]
pub mod hash;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub image: Option<String>,
    pub size: Size,
    pub scale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameData {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}