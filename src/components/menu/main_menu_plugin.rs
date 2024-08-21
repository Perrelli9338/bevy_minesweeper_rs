use bevy::prelude::*;
use crate::components::menu::{UISettings, MenuStates, cleanup, MenuButtonAction};
use crate::resources::assets::TextureAssets;

#[derive(Component)]
struct Menu;
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Main), Self::create)
            .add_systems(OnExit(MenuStates::Main), cleanup::<Menu>);
    }
}

impl MainMenu {
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
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
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
                            image: textures.icon.clone().into(),
                            style: Style {
                                width: Val::Px(64.),
                                ..default()
                            },
                            ..default()
                        });
                    });
            })
            .with_children(|children| {
                for (action, text) in [
                    (MenuButtonAction::Play, "Play"),
                    (MenuButtonAction::Settings, "Settings"),
                    (MenuButtonAction::Quit, "Exit"),
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
            }
            });
    }
    
}