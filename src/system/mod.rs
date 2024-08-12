use bevy::app::{App, Update};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{in_state, IntoSystemConfigs, MouseButton, OnEnter, Plugin};
use crate::GameState;
use crate::resources::events::*;
mod input;
mod uncover;
mod flagged;

pub struct SystemPlugins;

impl SystemPlugins {
}

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (

                input::input_handling,
                flagged::flag_tiles,
                uncover::trigger_event_handler,
                uncover::uncover_tiles,
            ).run_if(in_state(GameState::Playing)))
        .add_event::<TileTriggerEvent>()
        .add_event::<TileFlaggedEvent>()
        .add_event::<GameWin>()
        .add_event::<GameLose>();
    }
}