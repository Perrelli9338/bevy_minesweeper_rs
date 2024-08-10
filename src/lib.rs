#![allow(clippy::type_complexity)]

mod actions;

pub mod resources;

mod components;

use actions::ActionsPlugin;
use resources::audio::InternalAudioPlugin;
use resources::loading::LoadingPlugin;
use components::menu::MenuPlugin;
use components::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use crate::resources::BoardPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Close the game
    Close,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            BoardPlugin,
            ActionsPlugin,
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin
        ));
    }
}
