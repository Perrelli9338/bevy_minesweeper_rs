#![allow(clippy::type_complexity)]

pub mod resources;

mod system;

mod components;
use system::SystemPlugins;
use resources::audio::InternalAudioPlugin;
use components::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::*;
use crate::components::TimingPlugin;
use crate::resources::ResourcePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
#[derive(States)]
enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    Endgame,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Close the game
    Close,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins((
            ResourcePlugin,
            SystemPlugins,
            MenuPlugin,
            TimingPlugin
        ));
    }
}
