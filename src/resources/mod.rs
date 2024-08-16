use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, in_state, OnEnter};
use crate::actions::{Actions, set_movement_actions};
use crate::GameState;
use crate::resources::tile_map::TileMap;

pub(crate) mod tile;
pub(crate) mod audio;
pub(crate) mod loading;
pub(crate) mod tile_map;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::Playing), Self::create);
    }

}

impl BoardPlugin {
    pub fn create() {
        let mut tile_map = TileMap::new(20, 20);
        tile_map.set_bombs(40);
        #[cfg(debug_assertions)]
        log::info!("{}", tile_map.log());
    }
}