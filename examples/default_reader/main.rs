use bevy::prelude::*;
use bevy_mod_spritesheet::{Frame, SpriteSheet, SpriteSheetBundle, SpriteSheetOptions, SpriteSheetPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), SpriteSheetPlugin))
        .add_systems(Startup, (spawn_camera,load_sprite_sheet))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_sprite_sheet(asset_server: Res<AssetServer>, mut commands: Commands) {
    let sprite_sheet: Handle<SpriteSheet> = asset_server.load("gabe-idle-run.json");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(500.0)),
                ..default()
            },
            ..default()
        },
        SpriteSheetBundle {
            frame: Frame::name("gabe-idle-run 6.png".into()),
            options: SpriteSheetOptions {
                texture_loading: true,
            },
            sprite_sheet,
        }
    ));
}