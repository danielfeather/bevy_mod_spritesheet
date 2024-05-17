use std::default;

use serde::{Deserialize, Serialize};

pub mod array;
pub mod hash;

#[derive(Debug,Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: Option<String>,
    pub format: Format,
    pub size: Size,
    pub scale: String,
    pub frame_tags: Option<Vec<String>>,
    pub layers: Option<Vec<Layer>>,
    pub slices: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub blend_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameData {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug,Default, Serialize, Deserialize)]
pub enum Format {
    #[default]
    RGBA8888
}