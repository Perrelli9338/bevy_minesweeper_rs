use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use crate::components::menu::{UISettings, MenuStates, cleanup, MenuButtonAction};
use crate::resources::settings::GameSettings;

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
}


#[derive(Component)]
pub struct MenuSettings;

pub struct SettingsMenu;

impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Settings), Self::create)
            .add_systems(Update, Self::button_functions.run_if(in_state(MenuStates::Settings)))
            .add_systems(OnExit(MenuStates::Settings), cleanup::<MenuSettings>);
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
                MenuSettings,
            ))
            .with_children(|children| {
                children.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 64.,
                        ..default()
                    }
                ));
            })
            .with_children(|children| {
                for (first_action, second_action, text, value) in [
                    (SettingsMenuButtonAction::DecrementWidthBoard, SettingsMenuButtonAction::IncrementWidthBoard, "Width", config.map_size.0.to_string()),
                    (SettingsMenuButtonAction::DecrementHeightBoard, SettingsMenuButtonAction::IncrementHeightBoard, "Height", config.map_size.1.to_string()),
                    (SettingsMenuButtonAction::DecrementBombCount, SettingsMenuButtonAction::IncrementBombCount, "Bombs", config.bomb_count.to_string()),
                    (SettingsMenuButtonAction::SafeStartOff, SettingsMenuButtonAction::SafeStartOn, "Safe start", match config.easy_mode { true => "On", false => "Off" }.to_string()),
                ] {
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(15.),
                                padding: UiRect::all(Val::Px(15.)),
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
                            children
                                .spawn((
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
                            children.spawn(TextBundle::from_section(
                                value,
                                TextStyle {
                                    font_size: 42.,
                                    ..default()
                                }
                            ));
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
                }
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
                        MenuButtonAction::BackToMainMenu,
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

    fn button_functions(
        mut commands: Commands,
        mut query: Query<&mut Text>,
        mut interaction_query: Query<
            (
                &Interaction,
                &SettingsMenuButtonAction
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut options: ResMut<GameSettings>,
    ) {
        let config = &mut options;
        for (interaction, menu_button_action) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                    match menu_button_action {
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
                            if config.map_size.0 > 1 && config.bomb_count <= (config.map_size.0 * config.map_size.1) - config.bomb_count {
                                config.map_size.0 -= 1;
                            }
                        }
                        SettingsMenuButtonAction::DecrementHeightBoard => {
                            if config.map_size.1 > 1 && config.bomb_count <= (config.map_size.0 * config.map_size.1) - config.bomb_count {
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
                    }
                    let mut settings_values = vec![
                        match config.easy_mode {
                            true => "On",
                            false => "Off"
                        }.to_string(),
                        config.bomb_count.to_string(),
                        config.map_size.1.to_string(),
                        config.map_size.0.to_string(),
                    ];
                    for mut b in query.iter_mut().skip(4).step_by(4) {
                        b.sections[0].value = settings_values.pop().unwrap();
                    }
                commands.insert_resource(GameSettings {
                    map_size: config.map_size,
                    bomb_count: config.bomb_count,
                    position: config.clone().position,
                    tile_size: config.clone().tile_size,
                    tile_padding: config.tile_padding,
                    easy_mode: config.easy_mode,
                })
                }
        }
    }

}
