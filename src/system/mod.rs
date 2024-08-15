use bevy::app::{App, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, DetectChanges, EventReader, EventWriter, in_state, IntoSystemConfigs, MouseButton, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Time, Touches, Window, With};
use bevy::window::PrimaryWindow;
use crate::{AppState, resources, resources::GameState};
use crate::components::timer::GameTimer;
use crate::resources::board::Board;
use crate::resources::events::*;

mod uncover;
mod flagged;
mod achievements;

pub struct SystemPlugins;

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                game_state_handler, (
                    game_input_handling,
                    flagged::flag_tiles,
                    uncover::input_event,
                    uncover::uncover_tiles
            ).run_if(in_state(GameState::Playing)),
             ).run_if(in_state(AppState::Playing)))
        .add_event::<TileTriggerEvent>()
        .add_event::<TileFlaggedEvent>()
        .add_event::<GameWinEvent>()
        .add_event::<GameLoseEvent>();
    }
}

pub fn game_input_handling(
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut mouse_input: Res<ButtonInput<MouseButton>>,
    mut touch_input: Res<Touches>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut flag_trigger_ewr: EventWriter<TileFlaggedEvent>
) {
    let Ok(window) = window_primary_query.get_single() else { return };
    if let Some(touch_position) = touch_input.first_pressed_position() {
        if let Some(tile_coordinates) = board.press_position(window, touch_position) {
            tile_trigger_ewr.send(TileTriggerEvent{
                coordinates: tile_coordinates
            });
        }
    }
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position() {
            if let Some(tile_coordinates) = board.press_position(window, mouse_position) {
                tile_trigger_ewr.send(TileTriggerEvent{
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
    mut mouse_input: Res<ButtonInput<MouseButton>>,
    mut touch_input: Res<Touches>,
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
    for e in lose_evr.read() {
        game_state.set(GameState::Lose);
    }
    for e in win_evr.read() {
        game_state.set(GameState::Win);
    }
}