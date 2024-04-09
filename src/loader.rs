use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use thiserror::Error;

use crate::format::json::array::JsonArray;
use crate::SpriteSheet;

pub const SUPPORTED_EXTENSIONS: &[&str] = &["json"];

#[derive(Default)]
pub struct Loader;

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

            let format = serde_json::from_slice::<JsonArray>(raw.as_slice())?;

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
    #[error("An error occured while reading the asset data")]
    Io(#[from] std::io::Error),
    #[error("An error occured while parsing the sprite sheet")]
    JsonParseError(#[from] serde_json::Error),
}