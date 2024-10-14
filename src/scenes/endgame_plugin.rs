use crate::{
    components::{stopwatch::GameStopwatch, timer::GameTimer},
    game::{board::Board, events::EndgameEvent},
    resources::GameState,
    scenes::cleanup,
    system::input::endgame_input_handling,
    widgets::text::UiTextWidgetExt,
    AppState,
};
use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
};
use sickle_ui::prelude::*;

pub struct EndgameScene;

impl Plugin for EndgameScene {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), timer_endgame)
            .add_systems(
                Update,
                cleanup_board
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(GameState::Win)),
            )
            .add_systems(
                Update,
                cleanup_board
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(GameState::Lose)),
            )
            .add_systems(OnEnter(AppState::Endgame), create_scene_endgame)
            .add_systems(
                Update,
                (endgame_input_handling, exit).run_if(in_state(AppState::Endgame)),
            )
            .add_systems(OnExit(AppState::Endgame), cleanup::<Scene>)
            .add_event::<EndgameEvent>();
    }
}

pub fn exit(
    mut trigger_event: EventReader<EndgameEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _event in trigger_event.read() {
        app_state.set(AppState::Menu);
        game_state.set(GameState::Disabled);
    }
}

pub fn timer_endgame(mut commands: Commands) {
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn cleanup_board(
    mut commands: Commands,
    board: Res<Board>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.tick(time.delta()).finished() {
        commands.entity(board.entity).despawn_recursive();
        app_state.set(AppState::Endgame);
    }
}

#[derive(Component)]
struct Scene;

#[warn(unused_mut)]
pub fn create_scene_endgame(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut stopwatch: ResMut<GameStopwatch>,
) {
    let mut msg = "You've ".to_owned();
    msg.push_str(match game_state.get() {
        GameState::Lose => "lose!",
        GameState::Win => "win!",
        _ => "[This is an easter egg ;)]",
    });
    let min = stopwatch.total_time.as_secs() / 60;
    let secs = stopwatch.total_time.as_secs() - (60 * min);
    let time_msg = format!(
        "Played for {}:{:02},{:03}",
        min,
        secs,
        stopwatch.total_time.subsec_millis()
    );
    commands
        .ui_builder(UiRoot)
        .container(
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            |children| {
                children.text(&msg, Some(54.));
                children.text(&time_msg, Some(32.));
                children.text("Click to return to main menu", Some(21.));
            },
        )
        .insert(Scene);
}
