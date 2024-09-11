use crate::{
    components::timer::GameTimer,
    resources::{
        board::Board,
        events::{EndgameEvent, TileFlaggedEvent, TileTriggerEvent},
        GameState,
    },
};
use bevy::{
    app::App,
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonInput},
    prelude::*,
    window::PrimaryWindow,
};

pub struct InputHandling;

impl Plugin for InputHandling {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(0.15, TimerMode::Once)))
            .insert_resource(TouchStatus {
                first_touch: Default::default(),
                is_covered: true,
            })
            .add_systems(
                Update,
                (
                    handle_mouse.run_if(run_if_any_button_mouse_pressed),
                    handle_touch,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn run_if_any_button_mouse_pressed(mouse_input: EventReader<MouseButtonInput>) -> bool {
    !mouse_input.is_empty()
}

fn handle_mouse(
    board: Res<Board>,
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut flag_trigger_ewr: EventWriter<TileFlaggedEvent>,
) {
    let Ok(window) = window_primary_query.get_single() else {
        return;
    };
    if let Some(mouse_position) = window.cursor_position() {
        if let Some(tile_coordinates) = board.press_position(window, mouse_position) {
            if mouse_input.just_pressed(MouseButton::Left) {
                tile_trigger_ewr.send(TileTriggerEvent {
                    coordinates: tile_coordinates,
                });
            } else if mouse_input.just_pressed(MouseButton::Right) {
                flag_trigger_ewr.send(TileFlaggedEvent {
                    coordinates: tile_coordinates,
                });
            }
        }
    }
}

#[derive(Resource)]
struct TouchStatus {
    first_touch: Vec2,
    is_covered: bool,
}

pub fn handle_touch(
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut position: ResMut<TouchStatus>,
    mut timer: ResMut<GameTimer>,
    mut flag_trigger_ewr: EventWriter<TileFlaggedEvent>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut touch_events: EventReader<TouchInput>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Ok(window) = window_primary_query.get_single() else { return; };
    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Started {
            commands.insert_resource(TouchStatus {
                first_touch: touch.position,
                is_covered: true,
            });
            timer.0.reset();
        } else if let Some(tile_coordinates) = board.press_position(window, position.first_touch) {
            if timer.0.finished() && position.is_covered {
                commands.insert_resource(TouchStatus {
                    first_touch: touch.position,
                    is_covered: false,
                });
                flag_trigger_ewr.send(TileFlaggedEvent {
                    coordinates: tile_coordinates,
                });
            } else if touch.phase == TouchPhase::Ended {
                tile_trigger_ewr.send(TileTriggerEvent {
                    coordinates: tile_coordinates,
                });
            }
        }
    }
    timer.0.tick(time.delta());
}

pub fn endgame_input_handling(
    mouse_input: EventReader<MouseButtonInput>,
    touch_input: Res<Touches>,
    mut trigger_event: EventWriter<EndgameEvent>,
) {
    if touch_input.any_just_pressed() || !mouse_input.is_empty() {
        trigger_event.send(EndgameEvent);
    }
}
