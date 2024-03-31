use std::ops::Deref;

use bevy::prelude::*;
use crate::SpriteSheet;

/// Queries for `Entity`s without an `Handle<Image>` that have a loaded `Handle<SpriteSheet>` to see
/// if the image can be loaded using the path within the SpriteSheets metadata.
pub fn load_image() {
    todo!();
}


pub fn load_atlas(
    entities: Query<(Entity, &Handle<SpriteSheet>), Without<Handle<TextureAtlasLayout>>>,
    mut events: EventReader<AssetEvent<SpriteSheet>>,
    store: Assets<SpriteSheet>,
    mut commands: Commands,
) {
    for event in events.read() {
        for (entity, sprite_sheet_handle) in entities.iter() {
            if !event.is_loaded_with_dependencies(sprite_sheet_handle) {
                continue;
            }

            if let Some(sprite_sheet) = store.get(sprite_sheet_handle) {

                let format = &sprite_sheet.0;

                let size = &format.meta.size;
                let layout = TextureAtlasLayout::new_empty(Vec2::new(size.w, size.h));

                for frame in &format.frames {
                    layout.add_texture()
                }
            } else {
                error!("SpriteSheet is missing from store")
            }
        }
    }
}