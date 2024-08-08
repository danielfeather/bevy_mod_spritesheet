use crate::{format, Frame, SpriteSheet, SpriteSheetOptions};
use bevy::prelude::*;

/// Query that will get all entities with a Handle<SpriteSheet<T>> and where
/// Handle<SpriteSheet<T>> has changed (includes added)
pub type WithoutTextureAtlasLayoutQuery<'w, 's, T> =
    Query<'w, 's, (Entity, &'static Handle<SpriteSheet<T>>), Changed<Handle<SpriteSheet<T>>>>;

/// System responsible for creating and inserting the `Handle<TextureAtlasLayout>` on the
/// entity with a `Handle<SpriteSheet<T>>`
pub fn setup_layouts<T: format::SpriteSheetFormat + Send + Sync + TypePath + std::fmt::Debug>(
    entities: WithoutTextureAtlasLayoutQuery<'_, '_, T>,
    sprite_sheets: Res<Assets<SpriteSheet<T>>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    if entities.is_empty() {
        return;
    }

    for (entity, sprite_sheet_handle) in entities.iter() {
        let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) else {
            continue;
        };

        let layout_handle = layouts.add(sprite_sheet.create_layout());

        commands.entity(entity).insert(layout_handle);
    }
}

/// Query for getting all the entities where the
/// `Handle<TextureAtlasLayout>`s have changed (including
/// added)
pub type WithoutTextureAtlasQuery<'w, 'b, T> = Query<
    'w,
    'b,
    (
        Entity,
        &'static Frame,
        &'static Handle<SpriteSheet<T>>,
        &'static Handle<TextureAtlasLayout>,
    ),
    Changed<Handle<TextureAtlasLayout>>,
>;

/// System responsible for creating the `TextureAtlas` on
/// an entity, based on changed `Handle<TextureAtlasLayout>`s
pub fn setup_texture_atlases<
    T: format::SpriteSheetFormat + Send + Sync + TypePath + std::fmt::Debug,
>(
    entities: WithoutTextureAtlasQuery<'_, '_, T>,
    sprite_sheets: Res<Assets<SpriteSheet<T>>>,
    mut commands: Commands,
) {
    for (entity, frame, sprite_sheet_handle, layout) in entities.iter() {
        let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) else {
            continue;
        };

        let index = match sprite_sheet.get_sprite_index(frame) {
            Some(index) => index,
            None => {
                error!("Couldn't find frame: {}", **frame);
                0
            }
        };

        commands.entity(entity).insert(TextureAtlas {
            index,
            layout: layout.clone(),
        });
    }
}

/// System for loading the corresponding textures for the specified SpriteSheetFormat
/// if `texture_loading` is enabled and if the sprite sheet format supports it
pub fn load_textures<T: format::SpriteSheetFormat + Send + Sync + TypePath + std::fmt::Debug>(
    entities: Query<(Entity, &SpriteSheetOptions, &Handle<SpriteSheet<T>>)>,
    sprite_sheets: Res<Assets<SpriteSheet<T>>>,
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

        let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) else {
            continue;
        };

        if loaded.contains(&entity) {
            continue;
        }

        let Some(path) = sprite_sheet_handle.path() else {
            error!("Unable to determine path to sprite sheet");
            loaded.push(entity);
            continue;
        };

        let Some(image_path) = sprite_sheet.get_texture() else {
            error!("Unable to determine path to sprite sheet image");
            loaded.push(entity);
            continue;
        };

        let image_handle: Handle<Image> = match path.resolve_embed(image_path) {
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

pub type ChangedQuery<'w, 's, T> = Query<
    'w,
    's,
    (
        &'static Frame,
        &'static Handle<SpriteSheet<T>>,
        &'static mut TextureAtlas,
    ),
    Or<(Changed<Frame>, Changed<Handle<SpriteSheet<T>>>)>,
>;

/// System for watching for changes on `Frame` components so that the underlying `TextureAtlas` components
/// can be updated
pub fn detect_frame_changes<
    T: format::SpriteSheetFormat + Send + Sync + TypePath + std::fmt::Debug,
>(
    mut changed: ChangedQuery<'_, '_, T>,
    sprite_sheets: Res<Assets<SpriteSheet<T>>>,
) {
    for (frame, sprite_sheet_handle, mut atlas) in changed.iter_mut() {
        let Some(sprite_sheet) = sprite_sheets.get(sprite_sheet_handle) else {
            error!("SpriteSheet is missing from asset store`");
            continue;
        };

        let Some(index) = sprite_sheet.get_sprite_index(frame) else {
            error!("Couldn't find frame: {}", **frame);
            continue;
        };

        atlas.index = index
    }
}
