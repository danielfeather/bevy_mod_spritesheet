use bevy::prelude::*;
use bevy_mod_spritesheet::{Frame, SpriteSheet, SpriteSheetPlugin};

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
    let image: Handle<Image> = asset_server.load("gabe-idle-run.png");

    commands.spawn((
        SpriteBundle {
            texture: image,
            ..default()
        },
        Frame::name("gabe-idle-run 1.png".into()),
        sprite_sheet, 
    ));
}