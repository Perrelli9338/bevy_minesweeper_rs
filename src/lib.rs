#![allow(clippy::type_complexity)]

use system::SystemPlugins;
use scenes::MenuPlugin;
use bevy::prelude::*;
use bevy_touch_camera::{TouchCameraConfig, TouchCameraPlugin};
use crate::{
    resources::ResourcePlugin,
};
pub mod resources;

mod system;

mod components;
pub(crate) mod scenes;
mod widgets;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
#[derive(States)]
enum AppState {
    // Loading assets logic
    #[default]
    Loading,
    // Game logic
    Playing,
    // End of the game logic
    Endgame,
    // Menu logic
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins((
                TouchCameraPlugin { config: TouchCameraConfig {
                zoom_sensitivity: 0.1,
                min_scale: 0.1,
                max_scale: 8.,
                ..default()
                } },
                ResourcePlugin,
                SystemPlugins,
                MenuPlugin
            ));
    }
}
