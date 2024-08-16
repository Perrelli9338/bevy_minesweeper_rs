#![allow(clippy::type_complexity)]

pub mod resources;

mod system;

mod components;
use system::SystemPlugins;
use components::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::*;
use crate::components::TimingPlugin;
use crate::resources::ResourcePlugin;
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
            SystemPlugins,
            MenuPlugin,
            TimingPlugin
        ));
    }
}
