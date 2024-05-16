use crate::Frame;
use bevy::prelude::*;

pub trait SpriteSheet {
    fn get_sprite_index(&self, frame: &Frame) -> Option<usize>;
    fn into_layout(&self) -> TextureAtlasLayout;
    fn get_texture(&self) -> Option<String>;
}