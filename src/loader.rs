use std::marker::PhantomData;

use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use thiserror::Error;

use crate::format::SpriteSheetFormat;
use crate::SpriteSheet;

/// Supported extensions for sprite sheets.
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    #[cfg(any(feature = "json-array", feature = "json-hash"))]
    "json"
];

#[derive(Default)]
/// Generic loader for any sprite sheet format.
pub struct Loader<T: SpriteSheetFormat>(PhantomData<T>);

impl<T: Send + Sync + TypePath + SpriteSheetFormat> AssetLoader for Loader<T>
where T: SpriteSheetFormat {
    type Asset = SpriteSheet<T>;
    type Settings = ();
    type Error = LoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>
    ) -> Result<Self::Asset, Self::Error> {
            let mut raw = Vec::new();

            let _ = reader
                .read_to_end(&mut raw)
                .await?;

            let format = T::new(raw);

            Ok(SpriteSheet(format))
        
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
    #[error("An error occurred while parsing the sprite sheet format data, {0}")]
    ParseError(#[from] serde_json::Error),
}