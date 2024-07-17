<div align="center">

# Sprite Sheet Formats for Bevy

[![crates.io](https://img.shields.io/crates/v/bevy_mod_spritesheet)](https://crates.io/crates/bevy_mod_spritesheet)
[![docs.rs](https://docs.rs/bevy_mod_spritesheet/badge.svg)](https://docs.rs/bevy_mod_spritesheet)

Create [Bevy](https://github.com/bevyengine/bevy) `TextureAtlasLayout`s from sprite sheet formats

> ⚠️ This is still a work in progress, expect breaking changes on every minor release ⚠️

</div>

## Overview

To use sprite sheets within Bevy you have to create a `TextureAtlasLayout`, if the sprites are in grid form and are of equal size, then this is fairly straightforward to set up (see [example](https://github.com/bevyengine/bevy/blob/release-0.13.2/examples/2d/sprite_sheet.rs)), but sprite sheets come in different shapes and sizes and often times they a bit more complex.

While you can set up the texture atlas yourself, most likely you'll have all of the sprites and their positions defined in an accompanying metadata file.

At its core, this crate allows you to use the metadata file to build a `TextureAtlasLayout` for you.

## Highlights

- Supported formats: 
    - JSON Array
    - JSON Hash
- Choose a sprite by using the `Frame` component with the name of frame you want from the sprite sheet on an entity`
- Automatic loading of textures, if supported by the sprite sheet format.

## Getting Started

1. Add the `SpriteSheetPlugin` to your app.
```rs
use bevy_mod_spritesheet::SpriteSheetPlugin;
...
app.add_plugins(SpriteSheetPlugin)
...
```

2. Using `Res<AssetServer>`, `load` the sprite sheet file and corresponding texture and insert the handles onto an entity, along with the `Frame` component and a Bevy `SpriteBundle`.


> ⚠️ By default, automatic texture loading is turned off. If you want to enable this. Add the `SpriteSheetOptions` component and set `texture_loading` to `true`

```rs
use bevy_mod_spritesheet::{format::json::array::JsonArray, Frame, SpriteSheet, SpriteSheetBundle, SpriteSheetOptions, SpriteSheetPlugin};

let sprite_sheet: Handle<SpriteSheet<JsonArray>> = asset_server.load("gabe-idle-run.json");
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

/// Using `bevy_mod_spritesheet::SpriteSheetBundle`
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

```
3. The `Handle<TextureAtlasLayout>` and `TextureAtlas` will be created in the background and inserted on the entity. If the sprite doesn't render correctly, check the console for error messages.

## Example

Check out the `examples` folder for ideas on usage.

## Supported Bevy Versions

|`bevy`|`bevy_mod_spritesheet`|
|---|---|
|0.14|0.3, main|
|0.13|0.2|
|0.13|0.1|