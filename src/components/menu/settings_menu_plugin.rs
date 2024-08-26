use bevy::prelude::*;
use crate::{components::menu::{UISettings, MenuStates, cleanup, MenuButtonAction, ButtonColors}, resources::settings::GameSettings, AppState};
use crate::components::menu::ChangeState;

#[derive(Component)]
enum SettingsMenuButtonAction {
    DecrementBombCount,
    IncrementBombCount,
    IncrementWidthBoard,
    DecrementWidthBoard,
    IncrementHeightBoard,
    DecrementHeightBoard,
    SafeStartOn,
    SafeStartOff,
    TurnFlagOn,
    TurnFlagOff,
    IncreaseTimer,
    DecreaseTimer,
    BackToMainMenu,
}

#[derive(Component)]
pub struct SettingsValues;

#[derive(Component)]
pub struct MenuSettings;

pub struct SettingsMenu;

impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Settings), Self::create)
            .add_systems(Update, Self::settings_button_functions.run_if(in_state(MenuStates::Settings)))
            .add_systems(OnExit(MenuStates::Settings), cleanup::<MenuSettings>)
            .add_systems(Update, settings_button_colors.run_if(in_state(MenuStates::Settings)));
    }
}

fn settings_button_colors (
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &SettingsMenuButtonAction,
            Option<&ChangeState>,
        ),
        With<Button>,
    >,
    mut config: ResMut<GameSettings>,
) {
    for (interaction, mut color, button_colors, button_action, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = button_colors.normal.into();
            }
            Interaction::Pressed => {
                *color = button_colors.pressed.into();
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
        }
        match button_action {
                    SettingsMenuButtonAction::DecrementBombCount => {
                        if !(config.bomb_count > 1) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::IncrementBombCount => {
                        if !(config.bomb_count < (config.map_size.0 * config.map_size.1) - 1) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::IncrementWidthBoard => {
                        if config.map_size.0 <= 32 {
                        } else {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::DecrementWidthBoard => {
                        if !(config.map_size.0 > 1 && ((config.map_size.0 - 1) * config.map_size.1) > config.bomb_count) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::DecrementHeightBoard => {
                        if !(config.map_size.1 > 1 && (config.map_size.0 * (config.map_size.1 - 1)) > config.bomb_count) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::IncrementHeightBoard => {
                        if !(config.map_size.1 <= 32) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::SafeStartOn => {
                        if !config.easy_mode {
                        } else {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::SafeStartOff => {
                        if !config.easy_mode {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::DecreaseTimer => {
                        if !(config.timer_start > 0.) {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::IncreaseTimer => {
                        if config.timer_start < 3.0 {
                        } else {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::TurnFlagOn => {
                        if config.flag_mode {
                            *color = button_colors.disabled.into();
                        }
                    }
                    SettingsMenuButtonAction::TurnFlagOff => {
                        if !config.flag_mode || config.bomb_count == (config.map_size.0 * config.map_size.1) - 1 && config.easy_mode {
                            *color = button_colors.disabled.into();
                        }
                    }
            _ => {}
        }
    }
}


impl SettingsMenu {

    fn create(mut commands: Commands, options: Option<Res<GameSettings>>) {
        let config = match options {
            None => GameSettings::default(),
            Some(c) => c.clone(),
        };
        let settings = UISettings::default();
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        margin: UiRect::all(Val::Auto),
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.),
                        ..default()
                    },
                    ..default()
                },
                MenuSettings,
            ))
            .with_children(|children| {
                children.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 54.,
                        ..default()
                    }
                ));
            })
            .with_children(|children| {
                children.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        row_gap: Val::Px(5.),
                        width: Val::Percent(80.0),
                        ..default()
                    },
                    ..default()
                })
                    .with_children(|children| {
                        for (first_action, second_action, text, value) in [
                            (SettingsMenuButtonAction::DecrementWidthBoard, SettingsMenuButtonAction::IncrementWidthBoard, "Width", config.map_size.0.to_string()),
                            (SettingsMenuButtonAction::DecrementHeightBoard, SettingsMenuButtonAction::IncrementHeightBoard, "Height", config.map_size.1.to_string()),
                            (SettingsMenuButtonAction::DecrementBombCount, SettingsMenuButtonAction::IncrementBombCount, "Bombs", config.bomb_count.to_string()),
                            (SettingsMenuButtonAction::SafeStartOff, SettingsMenuButtonAction::SafeStartOn, "Safe start", match config.easy_mode { true => "On", false => "Off" }.to_string()),
                            (SettingsMenuButtonAction::TurnFlagOff, SettingsMenuButtonAction::TurnFlagOn, "Flag mode", match config.flag_mode { true => "On", false => "Off" }.to_string()),
                            (SettingsMenuButtonAction::DecreaseTimer, SettingsMenuButtonAction::IncreaseTimer, "Start delay", format!("{:.01}s", config.timer_start)),
                        ] {
                            children.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        justify_content: JustifyContent::SpaceBetween,
                                        width: Val::Percent(100.0),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                MenuSettings,
                            ))
                            .with_children(|children| {
                                    children.spawn(TextBundle::from_section(
                                        text,
                                        TextStyle {
                                            font_size: 42.,
                                            ..default()
                                        }
                                    ));
                                    children.spawn(NodeBundle {
                                        style: Style {
                                            display: Display::Flex,
                                            justify_content: JustifyContent::SpaceBetween,
                                            width: Val::Percent(30.0),
                                            column_gap: Val::Px(5.),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                        .with_children(|children| {
                                        children.spawn((
                                            ButtonBundle {
                                                style: settings.button_settings_style.clone(),
                                                background_color: settings.button_colors.clone().normal.into(),
                                                border_radius: settings.button_border_style.clone(),
                                                ..Default::default()
                                            },
                                            first_action,
                                            settings.button_colors.clone(),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "<",
                                                TextStyle {
                                                    ..default()
                                                }
                                            ));
                                        });
                                    children.spawn((TextBundle::from_section(
                                        value,
                                        TextStyle {
                                            font_size: 37.,
                                            ..default()
                                        },
                                    ), SettingsValues));
                                    children
                                        .spawn((
                                            ButtonBundle {
                                                style: settings.button_settings_style.clone(),
                                                background_color: settings.button_colors.normal.clone().into(),
                                                border_radius: settings.button_border_style.clone(),
                                                ..Default::default()
                                            },
                                            second_action,
                                            settings.button_colors.clone(),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                ">",
                                                TextStyle {
                                                    ..default()
                                                }
                                            ));
                                        });
                                });
                                });
                        }
                    });
            })
            .with_children(|children| {
                children
                    .spawn((
                        ButtonBundle {
                            style: settings.button_style.clone(),
                            background_color: settings.button_colors.normal.clone().into(),
                            border_radius: settings.button_border_style.clone(),
                            ..Default::default()
                        },
                        settings.button_colors.clone(),
                        SettingsMenuButtonAction::BackToMainMenu,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Close",
                            TextStyle {
                                ..default()
                            }
                        ));
                    });
            });

    }

    fn settings_button_functions(
        mut commands: Commands,
        mut query: Query<&mut Text, With<SettingsValues>>,
        mut interaction_query: Query<
            (
                &Interaction,
                &SettingsMenuButtonAction,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut config: ResMut<GameSettings>,
        mut menu_state: ResMut<NextState<MenuStates>>,
    ) {
        for (interaction, button_action) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                    match button_action {
                        SettingsMenuButtonAction::DecrementBombCount => {
                            if config.bomb_count > 1 {
                                config.bomb_count -= 1;
                            }
                        }
                        SettingsMenuButtonAction::IncrementBombCount => {
                            if config.bomb_count < (config.map_size.0 * config.map_size.1) - 1 {
                                config.bomb_count += 1;
                            }
                        }
                        SettingsMenuButtonAction::IncrementWidthBoard => {
                            if config.map_size.0 <= 32 {
                                config.map_size.0 += 1;
                            }
                        }
                        SettingsMenuButtonAction::DecrementWidthBoard => {
                            if config.map_size.0 > 1 && ((config.map_size.0 - 1) * config.map_size.1) > config.bomb_count {
                                config.map_size.0 -= 1;
                            }
                        }
                        SettingsMenuButtonAction::DecrementHeightBoard => {
                            if config.map_size.1 > 1 && (config.map_size.0 * (config.map_size.1 - 1)) > config.bomb_count {
                                config.map_size.1 -= 1;
                            }
                        }
                        SettingsMenuButtonAction::IncrementHeightBoard => {
                            if config.map_size.1 <= 32 {
                                config.map_size.1 += 1;
                            }
                        }
                        SettingsMenuButtonAction::SafeStartOn => {
                            config.easy_mode = true;
                        }
                        SettingsMenuButtonAction::SafeStartOff => {
                            config.easy_mode = false;
                        }
                        SettingsMenuButtonAction::DecreaseTimer => {
                            if config.timer_start > 0. {
                                config.timer_start = format!("{:.01}", config.timer_start - 0.1).parse::<f32>().unwrap();
                            }
                        }
                        SettingsMenuButtonAction::IncreaseTimer => {
                            if config.timer_start < 3.0 {
                                config.timer_start += 0.1;
                            }
                        }
                        SettingsMenuButtonAction::TurnFlagOn => {
                            config.flag_mode = true;
                        }
                        SettingsMenuButtonAction::TurnFlagOff => {
                            config.flag_mode = false;
                        }
                        SettingsMenuButtonAction::BackToMainMenu => {
                            menu_state.set(MenuStates::Main)
                        }
                    }
                }
        }
        if config.bomb_count == (config.map_size.0 * config.map_size.1) - 1 && config.easy_mode {
            config.flag_mode = true
        }
        let mut settings_values = vec![
                        format!("{:.01}s", config.timer_start),
                        match config.flag_mode {
                            true => "On",
                            false => "Off"
                        }.to_string(),
                        match config.easy_mode {
                            true => "On",
                            false => "Off"
                        }.to_string(),
                        config.bomb_count.to_string(),
                        config.map_size.1.to_string(),
                        config.map_size.0.to_string(),
                    ];
                    for mut b in query.iter_mut() {
                        b.sections[0].value = settings_values.pop().unwrap();
                    }
                commands.insert_resource(GameSettings {
                    map_size: config.map_size,
                    bomb_count: config.bomb_count,
                    position: config.clone().position,
                    tile_size: config.clone().tile_size,
                    tile_padding: config.tile_padding,
                    easy_mode: config.easy_mode,
                    timer_start: config.timer_start,
                    flag_mode: config.flag_mode,
                })
                }
        }
