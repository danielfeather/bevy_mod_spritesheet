use bevy::prelude::*;
use loader::Loader;
use systems::{detect_frame_changes, load_atlas, load_textures, setup_texture_atlases};
use crate::format::json::array::JsonArray;

pub mod format;
mod systems;
pub mod loader;
mod spritesheet;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Frame>()
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

#[derive(Debug, Component, Default, Reflect)]
pub struct Frame(String);

impl Frame {
    pub fn name(name: String) -> Self {
        Self(name)
    }
}

// #[derive(Asset, TypePath)]
// pub struct SpriteSheet(JsonArray);

#[derive(Debug, Asset, TypePath, Deref)]
pub struct SpriteSheet<T: Send + Sync + TypePath + spritesheet::SpriteSheet>(T);

#[derive(Debug, Component, Default)]
pub struct SpriteSheetOptions {
    /// Determines if the associated texture should be loaded.
    /// e.g. `image` meta property in JSON Hash/Array.
    /// Defaults to `false`.
    pub texture_loading: bool
}

#[derive(Debug, Bundle, Default)]
pub struct SpriteSheetBundle<T: spritesheet::SpriteSheet + Send + Sync + TypePath> {
    pub frame: Frame,
    pub sprite_sheet: Handle<SpriteSheet<T>>,
    pub options: SpriteSheetOptions,
}