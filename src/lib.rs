use bevy::prelude::*;
use loader::Loader;
use systems::{detect_frame_changes, load_atlas, setup_texture_atlases};
use crate::format::json::array::JsonArray;

pub mod format;
mod systems;
pub mod loader;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<SpriteSheet>()
            .init_asset_loader::<Loader>()
            .add_systems(Update, (load_atlas, setup_texture_atlases, detect_frame_changes));
    }
}

#[derive(Debug, Component)]
pub struct Frame(String);

impl Frame {
    pub fn name(name: String) -> Self {
        Self(name)
    }
}

#[derive(Asset, TypePath)]
pub struct SpriteSheet(JsonArray);

