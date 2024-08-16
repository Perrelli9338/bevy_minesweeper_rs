mod main_menu_plugin;
mod settings_menu_plugin;

use bevy::prelude::*;

use crate::AppState;
use crate::resources::settings::{GameSettings, TileSize::Fixed};

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    normal: Color,
    hovered: Color,
}

#[derive(Component)]
struct ChangeState(AppState);

#[derive(Component)]
pub struct UISettings {
    button_colors: ButtonColors,
    button_style: Style,
    button_border_style: BorderRadius,
    button_settings_style: Style,
}

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

impl Default for UISettings {
    fn default() -> Self {
        Self {
            button_colors: ButtonColors::default(),
            button_style: Style {
                width: Val::Px(140.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            button_settings_style: Style {
                width: Val::Px(25.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            button_border_style: BorderRadius::all(
                Val::Px(8.),
            )
        }
    }
}

pub struct MenuPlugin;

// This plugin is responsible for the game menu (containing only one button...)
// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuStates>()
            .add_plugins((
                main_menu_plugin::MainMenu,
                settings_menu_plugin::SettingsMenu
            ))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::Menu), menu_setup)
            .add_systems(Update, (menu_action, button_states).run_if(in_state(AppState::Menu)))
            .insert_resource(GameSettings {
                map_size: (8, 8),
                bomb_count: 10,
                tile_padding: 3.0,
                tile_size: Fixed(50.0),
                easy_mode: true,
                position: Default::default(),
            });
    }

}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuStates>>){
  menu_state.set(MenuStates::Main);
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

