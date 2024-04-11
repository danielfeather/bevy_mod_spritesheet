<div align="center">

# Sprite Sheet Formats for Bevy

[![crates.io](https://img.shields.io/crates/v/bevy_mod_spritesheet)](https://crates.io/crates/bevy_mod_spritesheet)
[![docs.rs](https://docs.rs/bevy_mod_spritesheet/badge.svg)](https://docs.rs/bevy_mod_spritesheet)

Create [Bevy](https://github.com/bevyengine/bevy) `TextureAtlasLayout`s from sprite sheet formats

> ⚠️ This is still a work in progress, expect breaking changes on every minor release ⚠️

</div>

## Highlights

- Currently supports JSON Array (more coming)
- Add a single `Frame` component with the name of frame you want from the sprite sheet


## Getting Started

1. Add the `SpriteSheetPlugin` to your app.
```rs
use bevy_mod_spritesheet::SpriteSheetPlugin;
...
.add_plugins(SpriteSheetPlugin)
...
```

2. Using `Res<AssetServer>`, `load` the sprite sheet file and corresponding texture and insert the handles onto an entity, along with the `Frame` component and a Bevy `SpriteBundle`.

```rs
let sprite_sheet: Handle<SpriteSheet> = asset_server.load("gabe-idle-run.json");
let image: Handle<Image> = asset_server.load("gabe-idle-run.png");

commands.spawn((
    SpriteBundle {
        texture: image,
        sprite: Sprite {
            custom_size: Some(Vec2::splat(500.0)),
            ..default()
        },
        ..default()
    },
    Frame::name("gabe-idle-run 6.png".into()),
    sprite_sheet, 
));
```
3. The `Handle<TextureAtlasLayout>` and `TextureAtlas` will be generated in the background and inserted on the entity.

## Example

Check out the basic example under `examples/default_reader`.