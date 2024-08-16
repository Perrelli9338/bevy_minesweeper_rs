use bevy::asset::Handle;
use bevy::prelude::{Font, Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "pixeled.ttf")]
    pub font: Handle<Font>
}

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
    pub covered_tile: Handle<Image>
}