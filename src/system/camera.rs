use crate::AppState;
use bevy::input::touch::TouchPhase;
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

pub fn handle_touch(mut camera: Query<&mut Transform, With<Camera>>, touch_input: Res<Touches>,
                    mut touch_events: EventReader<TouchInput>) {
    if touch_input.iter().count() == 2 {
        for _ in touch_events.read() {
            let fingers: Vec<_> = touch_input.iter().collect();
            let (x1, y1) = (fingers[0].position().x, fingers[0].position().y);
            let (x2, y2) = (fingers[1].position().x, fingers[1].position().y);
            let previous_position = Vec2::new(
                (fingers[0].previous_position().x + fingers[1].previous_position().x) / 2.0,
                (fingers[0].previous_position().y + fingers[1].previous_position().y) / 2.0,
            );
            let last_position = Vec2::new((x1 + x2) / 2.0, (y1 + y2) / 2.0);
            
            let delta = previous_position - last_position;
            for mut transform in camera.iter_mut() {
                transform.translation.x += delta.x;
                transform.translation.y -= delta.y;
            }
        }
    }
}
