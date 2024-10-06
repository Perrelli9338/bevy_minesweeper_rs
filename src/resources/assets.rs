use bevy::{
    asset::Handle,
    prelude::{Font, Image, Resource},
};
use bevy_asset_loader::asset_collection::AssetCollection;
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}
#[cfg(all(target_os = "android", target_os = "ios", target_os = "wasm32"))]
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "pixeled.ttf")]
    pub font: Handle<Font>,
}
#[cfg(all(target_os = "android", target_os = "ios", target_os = "wasm32"))]
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/icon.png")]
    pub icon: Handle<Image>,
    #[asset(path = "textures/bomb.png")]
    pub bomb: Handle<Image>,
    #[asset(path = "textures/flag.png")]
    pub flag: Handle<Image>,
    #[asset(path = "textures/tile_uncovered.png")]
    pub tile: Handle<Image>,
    #[asset(path = "textures/tile_covered.png")]
    pub covered_tile: Handle<Image>,
    #[asset(path = "textures/wrong.png")]
    pub wrong: Handle<Image>,
}
#[cfg(not(all(target_os = "android", target_os = "ios", target_os = "wasm32")))]
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "embedded://pixeled.ttf")]
    pub font: Handle<Font>,
}

#[cfg(not(all(target_os = "android", target_os = "ios", target_os = "wasm32")))]
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "embedded://textures/icon.png")]
    pub icon: Handle<Image>,
    #[asset(path = "embedded://textures/bomb.png")]
    pub bomb: Handle<Image>,
    #[asset(path = "embedded://textures/flag.png")]
    pub flag: Handle<Image>,
    #[asset(path = "embedded://textures/tile_uncovered.png")]
    pub tile: Handle<Image>,
    #[asset(path = "embedded://textures/tile_covered.png")]
    pub covered_tile: Handle<Image>,
    #[asset(path = "embedded://textures/wrong.png")]
    pub wrong: Handle<Image>,
}
