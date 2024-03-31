use serde::{Deserialize, Serialize};
use crate::format::json::{FrameData, Meta};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonArray {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub filename: String,
    pub frame: FrameData,
    pub rotated: bool,
    pub trimmed: bool,
}

