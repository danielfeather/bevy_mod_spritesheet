use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{format::{json::{FrameData, Meta}, SpriteSheetFormat}, loader::Loader, SpriteSheet};
use crate::systems::{detect_frame_changes, load_atlas, load_textures, setup_texture_atlases};

#[derive(Debug, Default, Serialize, Deserialize, TypePath)]
pub struct JsonArray {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub filename: String,
    pub frame: FrameData,
}

pub struct JsonArrayPlugin;

impl Plugin for JsonArrayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<SpriteSheet<JsonArray>>()
            .init_asset_loader::<Loader<JsonArray>>()
            .add_systems(Update, (
                load_atlas::<JsonArray>, 
                setup_texture_atlases::<JsonArray>, 
                detect_frame_changes::<JsonArray>, 
                load_textures::<JsonArray>,
            ));
            
    }
}

impl SpriteSheetFormat for JsonArray {
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
    
    fn new(raw:Vec<u8>)-> Self {
        serde_json::from_slice::<JsonArray>(raw.as_slice()).unwrap()
    }
}