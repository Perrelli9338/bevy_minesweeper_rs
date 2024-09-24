use bevy::{
    prelude::*,
    winit::WinitSettings
};
use sickle_ui::prelude::*;
use crate::{
    resources::assets::TextureAssets,
    scenes::{cleanup, MenuButtonAction, MenuStates, H1},
    widgets::{
        button::UiButtonWidgetExt,
        text::UiTextWidgetExt,
    }
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
                    NodeBundle::default(), |children| {
                        children.text("Minesweeper").insert(H1);
                        children.spawn(ImageBundle {
                            image: textures.icon.clone().into(),
                            ..default()
                        }).style().width(Val::Px(59.));
                    }).style().display(Display::Flex).flex_direction(FlexDirection::Row);
                parent.button_MainMenu( "Play", MenuButtonAction::Play);
                parent.button_MainMenu( "Settings", MenuButtonAction::Settings);
                #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android"), not(target_os = "ios")))]
                parent.button_MainMenu( "Quit", MenuButtonAction::Quit);
                parent.text(&format!("v{}-rc", env!("CARGO_PKG_VERSION")));
            }).insert(Menu);
        commands.insert_resource(WinitSettings::desktop_app());
    }
}
