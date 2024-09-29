use bevy::input::touch::TouchPhase;
use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraHandling;

impl Plugin for CameraHandling {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_mouse, handle_touch.after(dummy_third_finger)).run_if(in_state(AppState::Playing)),
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

pub fn dummy_third_finger(
    mut touches: Res<Touches>,
    mut touch_events: EventWriter<TouchInput>,
    window_primary_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Ok(window) = window_primary_query.get_single() else {
        return;
    };
    if touches.iter().count() == 2 {
        let mut fingers = touches
            .iter()
            .map(|touch| touch.position())
            .collect::<Vec<_>>();
        touch_events.send(TouchInput {
            phase: TouchPhase::Moved,
            position: Vec2::new(
                (fingers[0].x + fingers[1].x) / 2.0,
                (fingers[0].y + fingers[1].y) / 2.0,
            ),
            window: window,
            force: None,
            id: 2,
        });
    }
}

pub fn handle_touch(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut touches: Res<Touches>,
    mut touch_events: EventReader<TouchInput>,
) {
    if touches.iter().count() == 2 {
        let mut fingers = touches
            .iter()
            .map(|touch| touch.position())
            .collect::<Vec<_>>();
            for event in touch_events.read() {
                if event.id == 2 {
                    let delta = event.position - Vec2::new(
                        (fingers[0].x + fingers[1].x) / 2.0,
                        (fingers[0].y + fingers[1].y) / 2.0,
                    );
                    for mut transform in camera.iter_mut() {
                        transform.translation.x += delta.x;
                        transform.translation.y -= delta.y;
                    }
                }
        }
    }
}
