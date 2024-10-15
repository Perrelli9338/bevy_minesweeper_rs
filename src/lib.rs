#![allow(clippy::type_complexity)]

use system::SystemPlugins;
use scenes::MenuPlugin;
use game::BoardPlugin;
use resources::ResourcePlugin;
use bevy::prelude::*;
use bevy_touch_camera::{TouchCameraConfig, TouchCameraPlugin};
pub mod resources;

mod system;

mod components;
mod game;
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
                ResourcePlugin,
                TouchCameraPlugin { config: TouchCameraConfig {
                    min_scale: 0.2,
                    max_scale: 8.,
                    ..default()
                } },
                BoardPlugin,
                SystemPlugins,
                MenuPlugin
            ));
    }
}