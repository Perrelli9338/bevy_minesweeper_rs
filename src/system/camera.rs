use crate::AppState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraHandling;

impl Plugin for CameraHandling {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_mouse.run_if(in_state(AppState::Playing)),
        );
    }
}

pub fn handle_mouse(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut zoom: Query<&mut OrthographicProjection, With<Camera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
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

    for mut projection in zoom.iter_mut() {
        for ev in evr_scroll.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    if (projection.scale - ev.y * 0.1) > 0.0 {
                        projection.scale -= ev.y * 0.1;
                    }
                }
                _ => {}
            }
        }
    }
}
