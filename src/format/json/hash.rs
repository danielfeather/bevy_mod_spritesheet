use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::systems::{detect_frame_changes, load_textures, setup_layouts, setup_texture_atlases};
use crate::SpriteSheet;
use crate::{
    format::{
        json::{FrameData, Meta},
        SpriteSheetFormat,
    },
    loader::Loader,
};

#[derive(Serialize, Deserialize, TypePath, Default)]
/// JSON Hash sprite sheet format.
pub struct JsonHash {
    frames: HashMap<String, Frame>,
    meta: Meta,
    #[serde(skip)]
    frame_indexes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An individual frame in a JSON Hash sprite sheet.
pub struct Frame {
    frame: FrameData,
}

/// Plugin to add support for loading sprite sheets in JSON Hash format.
pub struct JsonHashPlugin;

impl Plugin for JsonHashPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheet<JsonHash>>()
            .init_asset_loader::<Loader<JsonHash>>()
            .add_systems(
                Update,
                (
                    setup_layouts::<JsonHash>,
                    setup_texture_atlases::<JsonHash>,
                    detect_frame_changes::<JsonHash>,
                    load_textures::<JsonHash>,
                ),
            );
    }
}

impl SpriteSheetFormat for JsonHash {
    fn get_sprite_index(&self, frame_name: &crate::Frame) -> Option<usize> {
        self.frame_indexes
            .iter()
            .position(|frame| frame == &frame_name.0)
    }

    fn create_layout(&self) -> TextureAtlasLayout {
        let size = &self.meta.size;
        let mut layout = TextureAtlasLayout::new_empty(UVec2::new(size.w, size.h));

        for frame in &self.frame_indexes {
            let data = &self.frames.get(frame).unwrap().frame;

            let rect = URect::new(data.x, data.y, data.x + data.w, data.y + data.h);
            layout.add_texture(rect);
        }

        layout
    }

    fn get_texture(&self) -> Option<&str> {
        self.meta.image.as_deref()
    }

    fn new(raw: Vec<u8>) -> Self {
        let mut format = serde_json::from_slice::<JsonHash>(raw.as_slice()).unwrap();

        let frame = &format.frames;

        for (name, _) in frame.iter() {
            format.frame_indexes.push(name.clone());
        }

        format
    }
}
