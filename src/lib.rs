use bevy::prelude::*;

#[cfg(feature = "json-array")]
use crate::format::json::array::JsonArrayPlugin;

#[cfg(feature = "json-hash")]
use format::json::hash::JsonHashPlugin;
/// Built-in formats for sprite sheets.
pub mod format;
/// Generic systems for loading sprite sheets.
mod systems;

/// Generic loader for any sprite sheet format.
pub mod loader;

/// Adds the necessary assets, loaders and systems to load sprite sheets.
pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Frame>();

        #[cfg(feature = "json-array")]
        app.add_plugins(JsonArrayPlugin);

        #[cfg(feature = "json-hash")]
        app.add_plugins(JsonHashPlugin);
    }
}

#[derive(Debug, Component, Default, Reflect, Deref, DerefMut)]
/// Component that represents a frame in a sprite sheet.
pub struct Frame(String);

impl Frame {
    pub fn name(name: String) -> Self {
        Self(name)
    }
}

#[derive(Debug, Asset, TypePath, Deref)]
/// Asset representing a sprite sheet stored in its associated format.
pub struct SpriteSheet<T: Send + Sync + TypePath + format::SpriteSheetFormat>(T);

#[derive(Debug, Component, Default)]
/// Options for loading a sprite sheet.
pub struct SpriteSheetOptions {
    /// Determines if the associated texture should be loaded.
    /// e.g. `image` meta property in JSON Hash/Array.
    /// Defaults to `false`.
    pub texture_loading: bool,
}

#[derive(Debug, Bundle, Default)]
/// Bundle of components needed to load a sprite sheet.
pub struct SpriteSheetBundle<T: format::SpriteSheetFormat + Send + Sync + TypePath> {
    pub frame: Frame,
    pub sprite_sheet: Handle<SpriteSheet<T>>,
    pub options: SpriteSheetOptions,
}
