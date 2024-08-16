use bevy::app::{App, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{EventReader, EventWriter, in_state, IntoSystemConfigs, MouseButton, NextState, Plugin, Query, Res, ResMut, Touches, Window, With};
use bevy::tasks::futures_lite::StreamExt;
use bevy::window::PrimaryWindow;
use crate::{AppState, resources::GameState};
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
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut flag_trigger_ewr: EventWriter<TileFlaggedEvent>
) {
    let Ok(window) = window_primary_query.get_single() else { return };
    let mut fingers = Vec::new();
    for finger in touch_input.iter(){
        if touch_input.just_pressed(finger.id()) {
            fingers.push(finger);
        }
        if fingers.len() >= 2 {
            if let touch_position = fingers.get(1).unwrap().position() {
                if let Some(tile_coordinates) = board.press_position(window, touch_position) {
                    flag_trigger_ewr.send(TileFlaggedEvent{
                        coordinates: tile_coordinates
                    });
                }
            }
        } else {
            if let Some(touch_position) = touch_input.first_pressed_position() {
                if let Some(tile_coordinates) = board.press_position(window, touch_position) {

                    tile_trigger_ewr.send(TileTriggerEvent{
                        coordinates: tile_coordinates
                    });
                }
            }
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
    }
    for _e in win_evr.read() {
        game_state.set(GameState::Win);
    }
}