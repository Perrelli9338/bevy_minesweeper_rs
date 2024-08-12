use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;
pub use coordinates::Coordinates;

mod coordinates;

pub(crate) mod menu;
pub use bomb::Bomb;
pub use bomb_neighbor::BombNeighbor;
pub use flag::flagged;

mod bomb;
mod bomb_neighbor;
pub(crate) mod uncover;
pub(crate) mod flag;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
