use bevy_embedded_assets::EmbeddedAssetPlugin;
use crate::{
    resources::assets::{FontAssets, TextureAssets},
    AppState,
};
use bevy::{
    app::{App, Plugin},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_asset_loader::{
    loading_state::{LoadingState, LoadingStateAppExt},
    prelude::ConfigureLoadingState,
};

pub(crate) mod assets;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EmbeddedAssetPlugin::default())
            .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Menu)
                .load_collection::<FontAssets>()
                .load_collection::<TextureAssets>(),
        )
        .init_state::<GameState>();
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub(crate) enum GameState {
    Win,
    Lose,
    Loading,
    Pause,
    Playing,
    #[default]
    Disabled,
}
