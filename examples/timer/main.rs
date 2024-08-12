use std::{ops::Index, time::Duration};

use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::texture::{ImageFormat, ImageFormatSetting, ImageLoaderSettings},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_spritesheet::{
    format::json::array::JsonArray, Frame, SpriteSheet, SpriteSheetBundle, SpriteSheetPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            SpriteSheetPlugin,
        ))
        .add_systems(Startup, (spawn_camera, load_sprite_sheet))
        .add_systems(Update, change_sprite_timer)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(ChangeSpriteTimer {
        timer: Timer::new(Duration::from_secs_f32(0.15), TimerMode::Repeating),
    });
    commands.spawn(Camera2dBundle::default());
}

fn load_sprite_sheet(asset_server: Res<AssetServer>, mut commands: Commands) {
    let sprite_sheet: Handle<SpriteSheet<JsonArray>> =
        asset_server.load("gabe-idle-run-array.json");

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                "gabe-idle-run.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.format = ImageFormatSetting::Format(ImageFormat::Png)
                },
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(500.0)),
                ..default()
            },
            ..default()
        },
        SpriteSheetBundle {
            frame: Frame::name("gabe-idle-run 6.png".into()),
            sprite_sheet,
            ..default()
        },
    ));
}

#[derive(Debug, Resource)]
struct ChangeSpriteTimer {
    timer: Timer,
}

const FRAMES: &'static [&str] = &[
    "gabe-idle-run 0.png",
    "gabe-idle-run 1.png",
    "gabe-idle-run 2.png",
    "gabe-idle-run 3.png",
    "gabe-idle-run 4.png",
    "gabe-idle-run 5.png",
    "gabe-idle-run 6.png",
];

fn change_sprite_timer(
    mut timer: ResMut<ChangeSpriteTimer>,
    time: Res<Time>,
    mut sprites: Query<&mut Frame>,
) {
    timer.timer.tick(time.delta());

    for mut frame in sprites.iter_mut() {
        if timer.timer.just_finished() {
            let index = FRAMES
                .iter()
                .position(|&frame_name| frame_name == **frame.as_ref())
                .unwrap();

            if index + 1 < FRAMES.len() {
                **frame = FRAMES[index + 1].to_owned();
                continue;
            }

            **frame = FRAMES[0].to_owned();
        }
    }
}
