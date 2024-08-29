use bevy::{
    asset::Handle,
    prelude::{Font, Image, Resource}
};
use bevy_asset_loader::asset_collection::AssetCollection;

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
    pub covered_tile: Handle<Image>,
    #[asset(path = "textures/bomb_tile/1.png")]
    pub bomb_neighbour_1: Handle<Image>,
    #[asset(path = "textures/bomb_tile/2.png")]
    pub bomb_neighbour_2: Handle<Image>,
    #[asset(path = "textures/bomb_tile/3.png")]
    pub bomb_neighbour_3: Handle<Image>,
    #[asset(path = "textures/bomb_tile/4.png")]
    pub bomb_neighbour_4: Handle<Image>,
    #[asset(path = "textures/bomb_tile/5.png")]
    pub bomb_neighbour_5: Handle<Image>,
    #[asset(path = "textures/bomb_tile/6.png")]
    pub bomb_neighbour_6: Handle<Image>,
    #[asset(path = "textures/bomb_tile/7.png")]
    pub bomb_neighbour_7: Handle<Image>,
    #[asset(path = "textures/bomb_tile/8.png")]
    pub bomb_neighbour_8: Handle<Image>,
    pub covered_tile: Handle<Image>,
    #[asset(path = "textures/wrong.png")]
    pub wrong: Handle<Image>
}