use bevy::app::{App, Update};
use bevy::input::ButtonInput;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{EventWriter, in_state, IntoSystemConfigs, MouseButton, OnEnter, Plugin, Query, Res, Touches, Window, With};
use bevy::window::PrimaryWindow;
use crate::GameState;
use crate::resources::board::Board;
use crate::resources::events::*;
use crate::resources::ResourcesPlugin;

mod uncover;
mod flagged;

pub struct SystemPlugins;

impl SystemPlugins {
}

impl Plugin for SystemPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                input_handling,
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

pub fn input_handling(
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