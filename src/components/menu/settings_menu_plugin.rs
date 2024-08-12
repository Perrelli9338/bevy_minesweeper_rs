use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use crate::components::menu::{ButtonColors, ChangeState, cleanup_menu, button_states, Menu, UISettings, OpenLink};
use crate::GameState;
use crate::resources::settings::GameSettings;
use crate::resources::settings::TileSize::Fixed;

pub struct settings_menu;

impl Plugin for settings_menu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), Self::create)
            .add_systems(Update, button_states.run_if(in_state(GameState::Settings)))
            .add_systems(Update, button_functions.run_if(in_state(GameState::Settings)))
            .add_systems(OnExit(GameState::Settings), cleanup_menu);
    }
}

impl settings_menu {

    fn create(mut commands: Commands, options: Option<Res<GameSettings>>) {
        let mut config = match options {
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
                Menu,
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
                    Menu,
                ))
                    .with_children(|children| {
                                let button_colors = ButtonColors::default();
                                children.spawn(TextBundle::from_section(
                                    "Bombs",
                                    TextStyle {
                                        font_size: 42.,
                                        ..default()
                                    }
                                ));
                                children
                                    .spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(25.0),
                                                height: Val::Px(50.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..Default::default()
                                            },
                                            background_color: button_colors.clone().normal.into(),
                                            border_radius: BorderRadius::new(
                                                Val::Px(settings.round_corner),
                                                Val::Px(settings.round_corner),
                                                Val::Px(settings.round_corner),
                                                Val::Px(settings.round_corner),
                                            ),
                                            ..Default::default()
                                        },
                                        button_colors.clone(),
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
                                config.bomb_count.to_string(),
                                TextStyle {
                                    font_size: 42.,
                                    ..default()
                                }
                         ));
                            children
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(25.0),
                                            height: Val::Px(50.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        background_color: button_colors.clone().normal.into(),
                                        border_radius: BorderRadius::new(
                                            Val::Px(settings.round_corner),
                                            Val::Px(settings.round_corner),
                                            Val::Px(settings.round_corner),
                                            Val::Px(settings.round_corner),
                                        ),
                                        ..Default::default()
                                    },
                                    button_colors.clone(),
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
            })
            .with_children(|children| {
                let button_colors = ButtonColors::default();
                children
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            border_radius: BorderRadius::new(
                                Val::Px(settings.round_corner),
                                Val::Px(settings.round_corner),
                                Val::Px(settings.round_corner),
                                Val::Px(settings.round_corner),
                            ),
                            ..Default::default()
                        },
                        button_colors,
                        ChangeState(GameState::Menu),

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

}

fn button_functions(mut next_state: ResMut<NextState<GameState>>,
                    mut interaction_query: Query<
                        (
                            &Interaction,
                            &mut BackgroundColor,
                            &ButtonColors,
                            Option<&ChangeState>,
                            Option<&OpenLink>,
                        ),
                        (Changed<Interaction>, With<Button>),
                    >,
                    mut options: ResMut<GameSettings>,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    let config = &mut options;
                    config.bomb_count -= 1;
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
