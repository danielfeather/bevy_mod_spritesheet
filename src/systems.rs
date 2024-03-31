use std::ops::Deref;

use bevy::prelude::*;
use crate::{Frame, SpriteSheet};

/// Queries for `Entity`s without an `Handle<Image>` that have a loaded `Handle<SpriteSheet>` to see
/// if the image can be loaded using the path within the SpriteSheets metadata.
pub fn load_image() {
    todo!();
}


pub fn load_atlas(
    entities: Query<(Entity, &Handle<SpriteSheet>), Without<Handle<TextureAtlasLayout>>>,
    mut events: EventReader<AssetEvent<SpriteSheet>>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    for event in events.read() {
        for (entity, sprite_sheet_handle) in entities.iter() {
            if !event.is_loaded_with_dependencies(sprite_sheet_handle) {
                continue;
            }

            if let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) {

                let format = &sprite_sheet.0;

                let size = &format.meta.size;
                let mut layout = TextureAtlasLayout::new_empty(Vec2::new(size.w, size.h));

                for frame in &format.frames {
                    let data = &frame.frame;

                    let rect = Rect::new(data.x, data.y, data.x + data.w, data.y + data.h);
                    layout.add_texture(rect);
                }

                let layout_handle = layouts.add(layout);

                commands.entity(entity).insert(layout_handle);
            } else {
                error!("SpriteSheet is missing from `Assets<SpriteSheet>`")
            }
        }
    }
}

pub fn setup_texture_atlases(
    entities: Query<(Entity, &Frame, &Handle<SpriteSheet>, &Handle<TextureAtlasLayout>), Without<TextureAtlas>>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    mut commands: Commands
) {
    for (entity, frame_name, sprite_sheet, layout) in entities.iter() {
        if let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet) {
            let frame = sprite_sheet.0.frames.iter().position(|frame | &frame.filename == &frame_name.0);

            if !frame.is_some() {
                error!("Couldn't find frame: {}", frame_name.0)
            }

            commands.entity(entity).insert(TextureAtlas {
                index: frame.unwrap(),
                layout: layout.clone(),
            });
        } else {
            error!("SpriteSheet is missing from `Assets<SpriteSheet>`")
        }
    }
}