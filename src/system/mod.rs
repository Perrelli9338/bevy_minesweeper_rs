use bevy::{
    app::{App, Update},
    input::ButtonInput,
    prelude::*,
    window::PrimaryWindow
};
use crate::{
    AppState, 
    resources::{
        GameState,
        events::*,
        board::Board,
        settings::GameSettings
    },
    components::{
        timer::GameTimer,
        stopwatch::GameStopwatch
    }
};

mod uncover;
mod flagged;
mod achievements;
pub(crate) mod cross_flag;

pub struct SystemPlugins;

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Menu), set_timer)
            .add_systems(OnEnter(GameState::Playing), GameStopwatch::new)
            .add_systems(OnExit(GameState::Playing), GameStopwatch::pause)
            .add_systems(Update, (
                game_state_handler, (
                    game_input_handling,
                    flagged::flag_tiles,
                    uncover::input_event,
                    uncover::uncover_tiles,
                    cross_flag::uncover_wrong_flags
            ).run_if(in_state(GameState::Playing)),
             ).run_if(in_state(AppState::Playing2D)))
        .add_event::<TileTriggerEvent>()
        .add_event::<TileFlaggedEvent>()
        .add_event::<GameWinEvent>()
        .add_event::<GameLoseEvent>();
    }
}

fn set_timer(mut commands: Commands, settings: Res<GameSettings>){
    commands.insert_resource(GameTimer(Timer::from_seconds(settings.timer_start, TimerMode::Once)));
}

pub fn game_input_handling(
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut flag_trigger_ewr: EventWriter<TileFlaggedEvent>,
) {
        let Ok(window) = window_primary_query.get_single() else { return };
        let mut fingers = Vec::new();
        for finger in touch_input.iter() {
            if touch_input.just_pressed(finger.id()) {
                fingers.push(finger);
            }
        }
        if fingers.len() >= 2 {
            let touch_position = fingers.first().unwrap().position();
            let touch_second_position = fingers.get(1).unwrap().position();
            if let Some(tile_coordinates) = board.press_position(window, touch_position) {
                flag_trigger_ewr.send(TileFlaggedEvent {
                    coordinates: tile_coordinates
                });
            } else if let Some(tile_coordinates) = board.press_position(window, touch_second_position) {
                flag_trigger_ewr.send(TileFlaggedEvent {
                    coordinates: tile_coordinates
                });
            }
        } else if let Some(touch_position) = fingers.first().unwrap().position() {
                if let Some(tile_coordinates) = board.press_position(window, touch_position) {
                    tile_trigger_ewr.send(TileTriggerEvent {
                        coordinates: tile_coordinates
                    });
                }
            }
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some(mouse_position) = window.cursor_position() {
                if let Some(tile_coordinates) = board.press_position(window, mouse_position) {
                    tile_trigger_ewr.send(TileTriggerEvent {
                        coordinates: tile_coordinates
                    });
                }
            }
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            if let Some(mouse_position) = window.cursor_position() {
                if let Some(tile_coordinates) = board.press_position(window, mouse_position) {
                    flag_trigger_ewr.send(TileFlaggedEvent{
                        coordinates: tile_coordinates
                    });
                }
            }
        }
    }

pub fn endgame_input_handling(
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    mut trigger_event: EventWriter<EndgameEvent>,
) {
    if touch_input.any_just_pressed()|| mouse_input.any_just_pressed([MouseButton::Right, MouseButton::Left, MouseButton::Middle]) {
        trigger_event.send(EndgameEvent);
    }
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