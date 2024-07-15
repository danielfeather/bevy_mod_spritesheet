
use serde::{Deserialize, Serialize};

#[cfg(feature = "json-array")]
/// Module for loading sprite sheets in the JSON Array format.
pub mod array;

#[cfg(feature = "json-hash")]
/// Module for loading sprite sheets in the JSON Hash format.
pub mod hash;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General meta data for a sprite sheet in a JSON based format.
pub struct Meta {
    pub image: Option<String>,
    pub size: Size,
    pub scale: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Position and dimensions for a single frame in a JSON based sprite sheet.
pub struct FrameData {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
// Size of the sprite sheets texture in pixels.
pub struct Size {
    pub w: u32,
    pub h: u32,
}