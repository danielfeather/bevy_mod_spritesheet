use bevy::{
    input::common_conditions::input_toggle_active,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_spritesheet::{
    Frame, SpriteSheet, SpriteSheetBundle, SpriteSheetOptions, SpriteSheetPlugin,
};
use bevy_web_asset::WebAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            WebAssetPlugin::default(),
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                }),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            SpriteSheetPlugin,
        ))
        .add_systems(Startup, (spawn_camera, load_sprite_sheet))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_sprite_sheet(asset_server: Res<AssetServer>, mut commands: Commands) {
    let sprite_sheet: Handle<SpriteSheet> = asset_server.load("https://raw.githubusercontent.com/danielfeather/bevy_mod_spritesheet/texture-loading/assets/gabe-idle-run.json");

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
        },
    ));
}