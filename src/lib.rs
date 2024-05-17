use bevy::prelude::*;

#[cfg(feature = "json-array")]
use crate::format::json::array::JsonArrayPlugin;

#[cfg(feature = "json-hash")]
use format::json::hash::JsonHashPlugin;

pub mod format;
mod systems;
pub mod loader;

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

#[derive(Debug, Component, Default, Reflect)]
pub struct Frame(String);

impl Frame {
    pub fn name(name: String) -> Self {
        Self(name)
    }
}

#[derive(Debug, Asset, TypePath, Deref)]
pub struct SpriteSheet<T: Send + Sync + TypePath + format::SpriteSheetFormat>(T);

#[derive(Debug, Component, Default)]
pub struct SpriteSheetOptions {
    /// Determines if the associated texture should be loaded.
    /// e.g. `image` meta property in JSON Hash/Array.
    /// Defaults to `false`.
    pub texture_loading: bool
}

#[derive(Debug, Bundle, Default)]
/// Bundle that has all the components 
pub struct SpriteSheetBundle<T: format::SpriteSheetFormat + Send + Sync + TypePath> {
    pub frame: Frame,
    pub sprite_sheet: Handle<SpriteSheet<T>>,
    pub options: SpriteSheetOptions,
}