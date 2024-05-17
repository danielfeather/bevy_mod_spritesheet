use std::marker::PhantomData;

use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use thiserror::Error;

use crate::format::SpriteSheetFormat;
use crate::SpriteSheet;

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    #[cfg(any(feature = "json-array", feature = "json-hash"))]
    "json"
];

#[derive(Default)]
pub struct Loader<T: SpriteSheetFormat>(PhantomData<T>);

impl<T: Send + Sync + TypePath + SpriteSheetFormat> AssetLoader for Loader<T>
where T: SpriteSheetFormat {
    type Asset = SpriteSheet<T>;
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

            let format = T::new(raw);

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
    #[error("An error occurred while parsing the sprite sheet format data")]
    ParseError(#[from] serde_json::Error),
}