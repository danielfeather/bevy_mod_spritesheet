pub mod json;

use crate::Frame;
use bevy::prelude::*;
use serde::Deserialize;

/// Trait for defining a format for a SpriteSheet
pub trait SpriteSheetFormat where Self: for<'de> Deserialize<'de> {
    /// Get the index of the sprite within the TextureAtlasLayout from the `Frame`
    fn get_sprite_index(&self, frame: &Frame) -> Option<usize>;

    /// Create TextureAtlasLayout from the format
    fn into_layout(&self) -> TextureAtlasLayout;

    /// Get the name of the texture associated with the SpriteSheet
    fn get_texture(&self) -> Option<&str>;

    /// Create a new instance of the format from raw bytes
    fn new(raw: Vec<u8>) -> Self {
        serde_json::from_slice::<Self>(raw.as_slice()).unwrap()
    }
}