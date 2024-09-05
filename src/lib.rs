#![allow(clippy::type_complexity)]

use system::SystemPlugins;
use scenes::MenuPlugin;
use bevy::prelude::*;
use crate::{
    components::TimingPlugin,
    resources::ResourcePlugin,
};
pub mod resources;

mod system;

mod components;
pub(crate) mod scenes;

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
