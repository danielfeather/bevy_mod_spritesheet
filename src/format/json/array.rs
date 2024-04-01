use serde::{Deserialize, Serialize};
use crate::format::json::{FrameData, Meta};

use super::Size;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonArray {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub filename: String,
    pub frame: FrameData,
    pub rotated: bool,
    pub trimmed: bool,
    pub source_size: Size,
    pub sprite_source_size: FrameData,
    pub duration: f32,
}

