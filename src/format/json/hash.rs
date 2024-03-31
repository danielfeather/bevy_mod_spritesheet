use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::format::json::Meta;
use crate::format::json::FrameData;

#[derive(Serialize, Deserialize)]
pub struct JsonHash {
    frames: HashMap<String, Frame>,
    meta: Meta,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    frame: FrameData,
    rotated: bool,
    trimmed: bool,
    sprite_source_size: FrameData,
    source_size: FrameData,
}