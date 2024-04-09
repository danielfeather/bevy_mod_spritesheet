use bevy::prelude::*;
use crate::{Frame, SpriteSheet, SpriteSheetOptions};

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
    mut commands: Commands,
) {
    for (entity, frame, sprite_sheet_handle, layout) in entities.iter() {
        if let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) {
            
            let index = get_sprite_index(frame, sprite_sheet);

            if index.is_none() {
                error!("Couldn't find frame: {}", frame.0)
            }

            commands.entity(entity).insert(TextureAtlas {
                index: index.unwrap(),
                layout: layout.clone(),
            });
        } else {
            error!("SpriteSheet is missing from `Assets<SpriteSheet>`")
        }
    }
}

pub fn load_textures(
    entities: Query<(Entity, &SpriteSheetOptions, &Handle<SpriteSheet>), Without<Handle<Image>>>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    mut commands: Commands,
) {
    for (entity, options, sprite_sheet_handle) in entities.iter() {
        if let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) {
            info!("{}", sprite_sheet_handle.path().unwrap());
        }
    }
}

pub fn detect_frame_changes(
    mut changed: Query<(&Frame, &Handle<SpriteSheet>, &mut TextureAtlas), Changed<Frame>>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
) {
    for (frame, sprite_sheet_handle, mut atlas) in changed.iter_mut() {
        let sprite_sheet = sprite_sheets.get(sprite_sheet_handle);

        if sprite_sheet.is_none() {
            error!("SpriteSheet is missing from `Assets<SpriteSheet>`")
        }

        let index = get_sprite_index(frame, sprite_sheet.unwrap());

        if index.is_none() {
            error!("Couldn't find frame: {}", frame.0)
        }

        atlas.index = index.unwrap()
    }
}

fn get_sprite_index(frame_name: &Frame, sprite_sheet: &SpriteSheet) -> Option<usize> {
    sprite_sheet.0.frames.iter().position(|frame | &frame.filename == &frame_name.0)
}