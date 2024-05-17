use crate::Frame;
use bevy::prelude::*;

pub trait SpriteSheetFormat {
    fn get_sprite_index(&self, frame: &Frame) -> Option<usize>;
    fn into_layout(&self) -> TextureAtlasLayout;
    fn get_texture(&self) -> Option<String>;
    fn new(raw:Vec<u8>)-> Self;
}