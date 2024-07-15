use crate::systems::{detect_frame_changes, load_atlas, load_textures, setup_texture_atlases};
use crate::{
    format::{
        json::{FrameData, Meta},
        SpriteSheetFormat,
    },
    loader::Loader,
    SpriteSheet,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, TypePath)]
/// JSON Array sprite sheet format.
pub struct JsonArray {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An individual frame in a JSON Array sprite sheet.
pub struct Frame {
    pub filename: String,
    pub frame: FrameData,
}

/// Plugin to add support for loading sprite sheets in JSON Array format.
pub struct JsonArrayPlugin;

impl Plugin for JsonArrayPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheet<JsonArray>>()
            .init_asset_loader::<Loader<JsonArray>>()
            .add_systems(
                Update,
                (
                    load_atlas::<JsonArray>,
                    setup_texture_atlases::<JsonArray>,
                    detect_frame_changes::<JsonArray>,
                    load_textures::<JsonArray>,
                ),
            );
    }
}

impl SpriteSheetFormat for JsonArray {
    fn get_sprite_index(&self, frame_name: &crate::Frame) -> Option<usize> {
        self.frames
            .iter()
            .position(|frame| &frame.filename == &**frame_name)
    }

    fn into_layout(&self) -> TextureAtlasLayout {
        let size = &self.meta.size;
        let mut layout = TextureAtlasLayout::new_empty(UVec2::new(size.w, size.h));

        for frame in &self.frames {
            let data = &frame.frame;

            let rect = URect::new(data.x, data.y, data.x + data.w, data.y + data.h);

            layout.add_texture(rect);
        }

        layout
    }

    fn get_texture(&self) -> Option<&str> {
        self.meta.image.as_ref().map(|string| string.as_str())
    }
}
