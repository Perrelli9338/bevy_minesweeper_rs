use bevy::prelude::*;
pub use coordinates::Coordinates;
pub use bomb::Bomb;
pub use bomb_neighbor::BombNeighbor;
pub mod coordinates;

mod bomb;
mod bomb_neighbor;
pub(crate) mod uncover;
pub(crate) mod flag;
pub(crate) mod timer;
pub(crate) mod stopwatch;
pub(crate) mod button_colors;
pub(crate) mod uisettings;
