use std::time::Duration;
use bevy::{
    prelude::*,
    text::TextSettings,
};
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use sickle_ui::SickleUiPlugin;
use crate::{
    AppState,
    resources::settings::GameSettings,
    components::button_colors::ButtonColors,
};
use crate::scenes::endgame_plugin::EndgameScene;

pub mod endgame_plugin;

mod main_menu_plugin;
mod settings_menu_plugin;
mod widgets;

#[derive(Component)]
struct ChangeState(AppState);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[derive(States)]
pub enum MenuStates {
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

pub struct MenuPlugin;

// This plugin is responsible for the game menu
// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuStates>()
            .add_plugins((
                SickleUiPlugin,
                main_menu_plugin::MainMenu,
                settings_menu_plugin::SettingsMenu,
                EndgameScene
            ))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::Menu), menu_setup)
            .add_systems(Update, text_size_change)
            .add_systems(Update, button_states.run_if(in_state(MenuStates::Main)))
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
            .insert_resource(GameSettings::default())
            .insert_resource(TextSettings {
                allow_dynamic_font_size: true,
                ..default()
            });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn menu_setup(
    mut menu_state: ResMut<NextState<MenuStates>>,
    mut commands: Commands,
) {
    menu_state.set(MenuStates::Main);
    commands.insert_resource(WinitSettings::desktop_app());
}

#[derive(Component)]
pub struct H1;

fn text_size_change(
    mut text: Query<&mut Text, With<H1>>,
    mut window: Query<&Window, With<PrimaryWindow>>
){
    if text.is_empty() || window.is_empty() { return }
    let width =  window.single_mut().width();
    for mut t in text.iter_mut() {
        t.sections[0].style.font_size = (8. * width) / 100.0;
    }
}

fn button_states(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_colors.pressed.into();
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuStates>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(AppState::Playing);
                    menu_state.set(MenuStates::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuStates::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuStates::Main),
            }
        }
    }
}

pub fn cleanup<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

