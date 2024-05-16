use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{format::json::{FrameData, Meta}, spritesheet::SpriteSheet};

use super::Size;

#[derive(Debug, Serialize, Deserialize, TypePath)]
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


impl SpriteSheet for JsonArray {
    fn get_sprite_index(&self, frame_name: &crate::Frame) -> Option<usize> {
        self
        .frames
        .iter()
        .position(|frame| &frame.filename == &frame_name.0)
    }
    fn into_layout(&self) -> TextureAtlasLayout {
        let size = &self.meta.size;
        let mut layout = TextureAtlasLayout::new_empty(Vec2::new(size.w, size.h));

        for frame in &self.frames {
            let data = &frame.frame;

            let rect = Rect::new(data.x, data.y, data.x + data.w, data.y + data.h);
            layout.add_texture(rect);
        }

        layout
    }
    fn get_texture(&self) -> Option<String> {
        self.meta.image.clone()
    }
}