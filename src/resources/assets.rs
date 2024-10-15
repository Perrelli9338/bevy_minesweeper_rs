use bevy::{
    asset::Handle,
    prelude::{Font, Image, Resource},
};
use bevy_asset_loader::asset_collection::AssetCollection;
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "embedded://pixeled.ttf")]
    pub font: Handle<Font>,
}

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
