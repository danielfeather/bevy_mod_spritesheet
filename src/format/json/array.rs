use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};
use crate::format::json::{FrameData, Meta};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonArray {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    filename: String,
    frame: FrameData,
    rotated: bool,
    trimmed: bool,
}

