use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod array;
pub mod hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: Format,
    pub size: Size,
    pub scale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameData {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    RGBA8888
}