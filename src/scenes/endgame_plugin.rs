use bevy::{
    app::{App, Plugin, Update},
    prelude::*
};
use crate::{scenes, AppState,
            components::{
                stopwatch::GameStopwatch,
                timer::GameTimer
            },
            system::input::endgame_input_handling,
            resources::{board::Board, events::EndgameEvent, GameState}
};
use scenes::cleanup;
pub struct EndgameScene;

impl Plugin for EndgameScene {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::Playing), timer_endgame)
            .add_systems(Update, cleanup_board.run_if(in_state(AppState::Playing)).run_if(in_state(GameState::Win)))
            .add_systems(Update, cleanup_board.run_if(in_state(AppState::Playing)).run_if(in_state(GameState::Lose)))
            .add_systems(OnEnter(AppState::Endgame), create_scene_endgame)
            .add_systems(Update, (endgame_input_handling, exit).run_if(in_state(AppState::Endgame)))
            .add_systems(OnExit(AppState::Endgame), cleanup::<Scene>)
            .add_event::<EndgameEvent>();
    }
}

pub fn exit(mut trigger_event: EventReader<EndgameEvent>, mut app_state: ResMut<NextState<AppState>>, mut game_state: ResMut<NextState<GameState>>) {
    for _event in trigger_event.read() {
        app_state.set(AppState::Menu);
        game_state.set(GameState::Disabled);
    }
}

pub fn timer_endgame(mut commands: Commands) {
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn cleanup_board(mut commands: Commands, board: Res<Board>, time: Res<Time>,
                     mut timer: ResMut<GameTimer>,
                     mut app_state: ResMut<NextState<AppState>>, ) {
    if timer.tick(time.delta()).finished() {
        commands.entity(board.entity).despawn_recursive();
        app_state.set(AppState::Endgame);
    }
}

#[derive(Component)]
struct Scene;

pub fn create_scene_endgame(mut commands: Commands, game_state: Res<State<GameState>>, mut stopwatch: ResMut<GameStopwatch>) {
    let mut msg = "You've ".to_owned();
    msg.push_str(match game_state.get() {
        GameState::Lose => "lose!",
        GameState::Win => "win!",
        _ => "[This text shouldn't be displayed, if you see it, let's say you've discovered an easter egg ;)]"
    });
    let time_msg = format!("Played for {}:{:02},{}", stopwatch.total_time.as_secs() / 60, stopwatch.total_time.as_secs(), stopwatch.total_time.subsec_millis());
    commands
        .spawn((
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
            Scene,
        ))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                msg,
                TextStyle {
                    font_size: 54.,
                    ..default()
                },
            ));
        })
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                time_msg,
                TextStyle {
                    font_size: 32.,
                    ..default()
                },
            ));
        })
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "Click to return to main menu",
                TextStyle {
                    font_size: 21.,
                    ..default()
                },
            ));
        });
}