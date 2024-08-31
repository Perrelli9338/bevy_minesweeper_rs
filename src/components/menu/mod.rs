use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use crate::{
    AppState,
    resources::settings::GameSettings
};
mod main_menu_plugin;
mod settings_menu_plugin;
mod selection_mode_plugin;

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    normal: Color,
    hovered: Color,
    pressed: Color,
    disabled: Color,
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
    SelectionMode,
    #[default]
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    GameIn2D,
    Settings,
    Selection,
    BackToMainMenu,
    GameIn3D,
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
        app
            .init_state::<MenuStates>()
            .add_plugins((
                PanOrbitCameraPlugin,
                main_menu_plugin::MainMenu,
                settings_menu_plugin::SettingsMenu,
                selection_mode_plugin::SelectionMenu
            ))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::Menu), menu_setup)
            .add_systems(Update, button_states.run_if(in_state(MenuStates::Main)))
            .add_systems(Update, button_states.run_if(in_state(MenuStates::SelectionMode)))
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
            .insert_resource(GameSettings::default());
    }

}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
            pressed: Color::linear_rgb(0.5, 0.5, 0.5),
            disabled: Color::linear_rgb(0.35, 0.35, 0.35),
        }
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn setup(mut commands: Commands){
    commands.spawn((Camera3dBundle {
        camera: Camera {
            is_active: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
                    PanOrbitCamera {
                        pan_sensitivity: 0.0,
                        zoom_upper_limit: Some(7.),
                        zoom_lower_limit: Some(2.),
                        ..default()
                    }, MainCamera)
    );
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
                MenuButtonAction::GameIn2D => {
                    game_state.set(AppState::Playing2D);
                    menu_state.set(MenuStates::Disabled);
                }
                MenuButtonAction::GameIn3D => {
                    game_state.set(AppState::Playing3D);
                    menu_state.set(MenuStates::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuStates::Settings),
                MenuButtonAction::Selection => menu_state.set(MenuStates::SelectionMode),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuStates::Main),
                MenuButtonAction::GameIn3D => {
                    game_state.set(AppState::Playing3D);
                    menu_state.set(MenuStates::Disabled);
                }
            }
        }
    }
}

pub fn cleanup<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

