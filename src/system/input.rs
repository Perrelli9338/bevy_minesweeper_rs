use crate::resources::board::Board;
use bevy::{
    prelude::{*, Query}
};
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::window::PrimaryWindow;
use crate::resources::events::{TileFlaggedEvent, TileTriggerEvent};

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