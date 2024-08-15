use bevy::app::{App, Plugin};
use bevy::prelude::*;
pub use coordinates::Coordinates;

pub mod coordinates;

pub(crate) mod menu;
pub use bomb::Bomb;
pub use bomb_neighbor::BombNeighbor;
use crate::{AppState, system};
use crate::components::menu::UISettings;
use crate::components::timer::GameTimer;
use crate::resources::board::Board;
use crate::resources::events::{EndgameEvent};
use crate::resources::GameState;

mod bomb;
mod bomb_neighbor;
pub(crate) mod uncover;
pub(crate) mod flag;
mod stopwatch;
pub(crate) mod timer;

pub struct TimingPlugin;

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::Playing), set_timer)
            .add_systems(Update, cleanup_board.run_if(in_state(AppState::Playing)).run_if(in_state(GameState::Win)))
            .add_systems(Update, cleanup_board.run_if(in_state(AppState::Playing)).run_if(in_state(GameState::Lose)))
            .add_systems(OnEnter(AppState::Endgame), create_scene_endgame)
            .add_systems(Update, (system::endgame_input_handling, exit).run_if(in_state(AppState::Endgame)))
            .add_systems(OnExit(AppState::Endgame), menu::cleanup::<Scene>)
            .add_event::<EndgameEvent>();
    }
}

fn exit(mut trigger_event: EventReader<EndgameEvent>, mut app_state: ResMut<NextState<AppState>>, mut game_state: ResMut<NextState<GameState>>){
    for event in trigger_event.read() {
        app_state.set(AppState::Menu);
        game_state.set(GameState::Disabled);
    }
}

fn set_timer(mut commands: Commands){
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn cleanup_board(mut commands: Commands, board: Res<Board>,  time: Res<Time>,
                 mut timer: ResMut<GameTimer>,
                 mut app_state: ResMut<NextState<AppState>>,) {
    if timer.tick(time.delta()).finished() {
        commands.entity(board.entity).despawn_recursive();
        app_state.set(AppState::Endgame);
    }
}

#[derive(Component)]
struct Scene;

fn create_scene_endgame(mut commands: Commands, game_state: Res<State<GameState>>) {
    let settings = UISettings::default();
    let mut msg = "You've ".to_owned();
    msg.push_str(&match game_state.get() {
        GameState::Lose => "lose",
        GameState::Win => "win",
        _ => ""
    }.to_owned());
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
                    font_size: 64.,
                    ..default()
                }
            ));
        })
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "Click to return to main menu",
                TextStyle {
                    font_size: 21.,
                    ..default()
                }
            ));
        });
}