use std::marker::PhantomData;

use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy::reflect::erased_serde::Serialize;
use serde::Deserialize;
use thiserror::Error;

use crate::{spritesheet, SpriteSheet};

pub const SUPPORTED_EXTENSIONS: &[&str] = &["json"];

pub struct Loader<T>(PhantomData<T>);

impl AssetLoader for Loader {
    type Asset = SpriteSheet;
    type Settings = ();
    type Error = LoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {

            let mut raw = Vec::new();

            let _ = reader
                .read_to_end(&mut raw)
                .await?;

            let format = serde_json::from_slice::<T>(raw.as_slice())?;

            Ok(SpriteSheet(format))
        })
    }

    fn extensions(&self) -> &[&str] {
        SUPPORTED_EXTENSIONS
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("An error occurred while reading the asset data")]
    Io(#[from] std::io::Error),
    #[error("An error occurred while parsing the sprite sheet")]
    JsonParseError(#[from] serde_json::Error),
}