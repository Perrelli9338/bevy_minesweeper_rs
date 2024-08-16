use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;
pub use coordinates::Coordinates;

mod coordinates;

pub(crate) mod menu;
pub(crate) mod player;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
