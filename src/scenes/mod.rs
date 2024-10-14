use bevy::{
    prelude::*,
    window::PrimaryWindow
};
use bevy_touch_camera::TouchCameraTag;
use sickle_ui::SickleUiPlugin;
use crate::{
    AppState,
    game::settings::GameSettings,
    components::button_colors::ButtonColors,
    scenes::endgame_plugin::EndgameScene,
};

pub mod endgame_plugin;
mod main_menu_plugin;
pub mod settings_menu_plugin;

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
pub enum MenuButtonAction {
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
            .add_systems(Update, (button_states.run_if(in_state(MenuStates::Main)), menu_action, text_size_change).run_if(in_state(AppState::Menu)))
            .insert_resource(GameSettings::default()) ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), TouchCameraTag));
}

fn menu_setup(
    mut menu_state: ResMut<NextState<MenuStates>>,
    mut commands: Commands,
) {
    menu_state.set(MenuStates::Main);
}

#[derive(Component)]
pub struct H1;

fn text_size_change(
    mut header: Query<&mut Text, With<H1>>,
    mut text: Query<&mut Text, Without<H1>>,
    mut window: Query<&Window, With<PrimaryWindow>>
){
    if header.is_empty() || window.is_empty() || text.is_empty() { return }
    let width = (12. * window.single_mut().width()) / 100.0;
    for mut t in header.iter_mut() {
        t.sections[0].style.font_size = match width {
            0.0..70.0 => width,
            _ => 45.
        };
    }
    for mut t in text.iter_mut() {
        t.sections[0].style.font_size = match width {
            0.0..60.0 => width / 2.,
            60.0..100.0 => width / 4.,
            _ => 21.
        };
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

