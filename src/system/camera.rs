use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraHandling;

impl Plugin for CameraHandling {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_mouse, handle_touch).run_if(in_state(AppState::Playing)),
        );
    }
}

pub fn handle_mouse(
    mut camera: Query<&mut Transform, With<Camera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let Ok(window) = window_primary_query.get_single() else {
        return;
    };
    if mouse_input.pressed(MouseButton::Middle) {
        for event in cursor_moved_events.read() {
            if let Some(last_position) = window.cursor_position() {
                let delta = event.position - last_position;
                for mut transform in camera.iter_mut() {
                    transform.translation.x += delta.x;
                    transform.translation.y -= delta.y;
                }
            }
        }
    }
}

pub fn handle_touch(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut touches: Res<Touches>,
    mut touch_events: EventReader<TouchInput>,
) {
    if touches.iter().count() == 3 {
        if let Some(last_position) = touches.iter().nth(1) {
            for event in touch_events.read().nth(1) {
                let delta = event.position - last_position.position();
                for mut transform in camera.iter_mut() {
                    transform.translation.x += delta.x;
                    transform.translation.y -= delta.y;
                }
            }
        }
    }
}
