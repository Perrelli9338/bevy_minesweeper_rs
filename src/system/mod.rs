use bevy::{
    app::{App, Update},
    prelude::*,
};
use crate::{
    AppState,
    resources::{
        GameState,
        events::*,
        settings::GameSettings,
    },
    components::{
        timer::GameTimer,
        stopwatch::GameStopwatch,
    },
};
use crate::system::input::InputHandling;

mod uncover;
mod flagged;
mod achievements;
pub(crate) mod cross_flag;
pub(crate) mod input;

pub struct SystemPlugins;

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Menu), set_timer)
            .add_systems(OnEnter(GameState::Playing), GameStopwatch::new)
            .add_systems(OnExit(GameState::Playing), GameStopwatch::pause)
            .add_systems(Update, (
                game_state_handler,
                flagged::flag_tiles,
                uncover::uncover_tiles,
                cross_flag::uncover_wrong_flags,
                uncover::input_event.run_if(in_state(GameState::Playing)),
            ).run_if(in_state(AppState::Playing)))
            .add_plugins(
                InputHandling
            )
            .add_event::<TileTriggerEvent>()
            .add_event::<TileFlaggedEvent>()
            .add_event::<GameWinEvent>()
            .add_event::<GameLoseEvent>();
    }
}

fn set_timer(mut commands: Commands, settings: Res<GameSettings>) {
    commands.insert_resource(GameTimer(Timer::from_seconds(settings.timer_start, TimerMode::Once)));
}

pub fn game_state_handler(
    mut lose_evr: EventReader<GameLoseEvent>,
    mut win_evr: EventReader<GameWinEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _e in lose_evr.read() {
        game_state.set(GameState::Lose);
        break;
    }
    for _e in win_evr.read() {
        game_state.set(GameState::Win);
        break;
    }
}