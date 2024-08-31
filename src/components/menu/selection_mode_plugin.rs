use bevy::prelude::*;
use crate::components::menu::{UISettings, MenuStates, cleanup, MenuButtonAction};
use crate::resources::assets::TextureAssets;

#[derive(Component)]
struct Menu;
pub struct SelectionMenu;

impl Plugin for SelectionMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::SelectionMode), Self::create)
            .add_systems(OnExit(MenuStates::SelectionMode), cleanup::<Menu>);
    }
}

impl SelectionMenu {
    fn create(mut commands: Commands, textures: Res<TextureAssets>) {
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
                        ..default()
                    },
                    ..default()
                },
                Menu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Select game mode",
                    TextStyle {
                        font_size: 41.,
                        ..default()
                    }
                ));
            })
            .with_children(|children| {
                children.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(50.0),
                            padding: UiRect::all(Val::Px(50.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    Menu,
                ))
                    .with_children(|children| {
                        for (action, text) in [
                            (MenuButtonAction::GameIn2D, "2D Mode"),
                            (MenuButtonAction::GameIn3D, "3D Mode"),
                        ] {
                            children
                                .spawn((
                                    ButtonBundle {
                                        style: settings.button_style.clone(),
                                        background_color: settings.button_colors.clone().normal.into(),
                                        border_radius: settings.button_border_style.clone(),
                                        ..Default::default()
                                    },
                                    settings.button_colors.clone(),
                                    action
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text,
                                        TextStyle {
                                            ..default()
                                        }
                                    ));
                                });
                        }});
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
    
}