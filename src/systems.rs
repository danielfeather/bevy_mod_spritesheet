use crate::{Frame, SpriteSheet, SpriteSheetOptions};
use bevy::prelude::*;

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
    entities: Query<
        (
            Entity,
            &Frame,
            &Handle<SpriteSheet>,
            &Handle<TextureAtlasLayout>,
        ),
        Without<TextureAtlas>,
    >,
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
    entities: Query<(Entity, &SpriteSheetOptions, &Handle<SpriteSheet>)>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    mut loaded: Local<Vec<Entity>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    if entities.is_empty() {
        return;
    }

    for (entity, options, sprite_sheet_handle) in entities.iter() {
        if !options.texture_loading {
            continue;
        }

        if let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) {
            if !loaded.contains(&entity) {
                let path = sprite_sheet_handle.path();

                if path.is_none() {
                    error!("Unable to determine path to sprite sheet");
                    loaded.push(entity);
                    continue;
                }

                let image_path = sprite_sheet.0.meta.image.as_ref();

                if image_path.is_none() {
                    debug!("{:?}", image_path);
                    error!("Unable to determine path to sprite sheet image");
                    loaded.push(entity);
                    continue;
                }

                let resolved_image_path = path.unwrap().resolve_embed(image_path.unwrap().as_ref());

                let image_handle: Handle<Image> = match resolved_image_path {
                    Ok(resolved) => server.load(resolved),
                    Err(e) => {
                        error!("Unable to resolve path to sprite sheet image, {}", e);
                        loaded.push(entity);
                        continue;
                    }
                };

                commands.entity(entity).insert(image_handle);
                loaded.push(entity);
            }
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
            error!("SpriteSheet is missing from `Assets<SpriteSheet>`");
            continue;
        }

        let index = get_sprite_index(frame, sprite_sheet.unwrap());

        if index.is_none() {
            error!("Couldn't find frame: {}", frame.0);
            continue;
        }

        atlas.index = index.unwrap()
    }
}

fn get_sprite_index(frame_name: &Frame, sprite_sheet: &SpriteSheet) -> Option<usize> {
    sprite_sheet
        .0
        .frames
        .iter()
        .position(|frame| &frame.filename == &frame_name.0)
}
