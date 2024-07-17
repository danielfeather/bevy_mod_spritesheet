# Changelog

All notable changes will be documented in this file.

## 0.3.0

- Updated Bevy to 0.14
- Changed function name of `into_layout` to `create_layout` to better represent its function
- Refactoring and code cleanliness
- `TextureAtlas` components will now be inserted with an index of `0` if the `Frame` doesn't exist within the spritesheet.

## 0.2.0

### Added
- JSON Hash Support
- Texture Loading Support

### Fixed
- [#1](https://github.com/danielfeather/bevy_mod_spritesheet/issues/1) Stack overflow on `LoaderError`

## 0.1.0

- Initial release
