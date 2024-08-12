use bevy::prelude::*;
use crate::components::menu::{ButtonColors, ChangeState, cleanup_menu, button_states, Menu, set_camera, UISettings};
use crate::GameState;
use crate::resources::loading::TextureAssets;

pub struct main_menu;

impl Plugin for main_menu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), Self::create)
            .add_systems(Update, button_states.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

impl main_menu {
    fn create(mut commands: Commands, textures: Res<TextureAssets>) {
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
                        children.spawn(TextBundle::from_section(
                            "Minesweeper",
                            TextStyle {
                                font_size: 64.,
                                ..default()
                            }
                        ));
                        children.spawn(ImageBundle {
                            image: textures.bevy.clone().into(),
                            style: Style {
                                width: Val::Px(64.),
                                ..default()
                            },
                            ..default()
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
                        ChangeState(GameState::Playing),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                ..default()
                            }
                        ));
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
                        ChangeState(GameState::Settings),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Settings",
                            TextStyle {
                                ..default()
                            }
                        ));
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
                        ChangeState(GameState::Close),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                ..default()
                            }
                        ));
                    });
            });
    }
    
}