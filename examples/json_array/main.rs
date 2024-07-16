use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_spritesheet::{
    format::json::array::JsonArray, Frame, SpriteSheet, SpriteSheetBundle, SpriteSheetOptions,
    SpriteSheetPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
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
    let sprite_sheet: Handle<SpriteSheet<JsonArray>> =
        asset_server.load("gabe-idle-run-array.json");

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
