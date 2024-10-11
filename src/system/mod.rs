use crate::system::input::InputHandling;
use crate::{
    components::{stopwatch::GameStopwatch, timer::GameTimer},
    resources::{events::*, settings::GameSettings, GameState},
    AppState,
};
use bevy::{
    app::{App, Update},
    prelude::*,
};
use crate::system::camera::CameraHandling;

mod achievements;
pub(crate) mod cross_flag;
mod flagged;
pub(crate) mod input;
mod uncover;
mod camera;

pub struct SystemPlugins;

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Menu), set_timer)
            .add_systems(OnEnter(GameState::Playing), GameStopwatch::new)
            .add_systems(OnExit(GameState::Playing), GameStopwatch::pause)
            .add_systems(
                Update,
                (
                    game_state_handler,
                    flagged::flag_tiles,
                    uncover::uncover_tiles,
                    cross_flag::uncover_wrong_flags,
                    uncover::input_event.run_if(in_state(GameState::Playing)),
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_plugins((InputHandling, CameraHandling))
            .add_event::<TileTriggerEvent>()
            .add_event::<TileFlaggedEvent>()
            .add_event::<GameWinEvent>()
            .add_event::<GameLoseEvent>();
    }
}

fn set_timer(mut commands: Commands, settings: Res<GameSettings>) {
    commands.insert_resource(GameTimer(Timer::from_seconds(
        settings.timer_start,
        TimerMode::Once,
    )));
}

pub fn game_state_handler(
    mut lose_evr: EventReader<GameLoseEvent>,
    mut win_evr: EventReader<GameWinEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !lose_evr.is_empty() {
        game_state.set(GameState::Lose);
    } else if !win_evr.is_empty() {
        game_state.set(GameState::Win);
    }
}
