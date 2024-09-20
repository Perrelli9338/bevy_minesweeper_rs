use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::{
    resources::assets::TextureAssets,
    scenes::{cleanup, MenuButtonAction, MenuStates},
    components::uisettings::UISettings,
};

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
        commands.ui_builder(UiRoot).container(
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
            |parent| {
                parent.container(
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    }, |children| {
                        children.spawn(TextBundle::from_section(
                            "Minesweeper",
                            TextStyle {
                                font_size: 59.,
                                ..default()
                            },
                        ));
                        children.spawn(ImageBundle {
                            image: textures.icon.clone().into(),
                            style: Style {
                                width: Val::Px(59.),
                                ..default()
                            },
                            ..default()
                        });
                    });
                for (action, text) in [
                    (MenuButtonAction::Play, "Play"),
                    (MenuButtonAction::Settings, "Settings"),
                    #[cfg(not(target_arch = "wasm32"))]
                    #[cfg(not(target_os = "android"))]
                    #[cfg(not(target_os = "ios"))]
                    (MenuButtonAction::Quit, "Exit"),
                ] {
                    parent
                        .container((
                                       ButtonBundle {
                                           style: settings.button_style.clone(),
                                           background_color: settings.button_colors.clone().normal.into(),
                                           border_radius: settings.button_border_style.clone(),
                                           ..Default::default()
                                       },
                                       settings.button_colors.clone(),
                                       action
                                   ), |children| {
                            children.spawn(TextBundle::from_section(
                                text,
                                TextStyle {
                                    ..default()
                                },
                            ));
                        });
                }
                parent.spawn(TextBundle::from_section(
                    format!("v{}-rc", env!("CARGO_PKG_VERSION")),
                    TextStyle {
                        ..default()
                    },
                ));
            }).insert(Menu);
    }
}