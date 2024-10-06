use crate::{
    scenes::{settings_menu_plugin::SettingsMenuButtonAction, MenuButtonAction},
    widgets::text::UiTextWidgetExt,
};
use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::components::button_colors::ButtonColors;

#[derive(Component)]
struct ButtonWidget;

pub trait UiButtonWidgetExt {
    fn button_main_menu(&mut self, text: &str, action: MenuButtonAction) -> UiBuilder<'_, Entity>;
    fn button_settings_menu(&mut self, text: &str, action: SettingsMenuButtonAction) -> UiBuilder<'_, Entity>;
}

impl UiButtonWidgetExt for UiBuilder<'_, Entity> {
    fn button_settings_menu(&mut self, text: &str, action: SettingsMenuButtonAction) -> UiBuilder<'_, Entity> {
        self.container(
            ((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: ButtonColors::default().normal.into(),
                    border_radius: BorderRadius::all(
                        Val::Px(8.),
                    ),
                    ..Default::default()
                },
                ButtonColors::default(),
                action,
            ),
                ButtonWidget
            ),
            |children| {
                children.text(text, None);
            },
        )
    }

    fn button_main_menu(&mut self, text: &str, action: MenuButtonAction) -> UiBuilder<'_, Entity> {
        self.container(
            ((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: ButtonColors::default().normal.into(),
                    border_radius: BorderRadius::all(
                        Val::Px(8.),
                    ),
                    ..Default::default()
                },
                ButtonColors::default(),
                action,
            ),
             ButtonWidget
            ),
            |children| {
                children.text(text, None);
            },
        )
    }
}
