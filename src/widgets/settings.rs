use crate::{
    scenes::{settings_menu_plugin::{SettingsMenuButtonAction, SettingsValues}},
    widgets::button::UiButtonWidgetExt,
    widgets::text::UiTextWidgetExt,
};
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
struct SettingsWidget;

pub trait UiSettingsWidgetExt {
    fn settings(
        &mut self,
        first_action: SettingsMenuButtonAction,
        second_action: SettingsMenuButtonAction,
        text: &str,
        value: &str,
    ) -> UiBuilder<'_, Entity>;
}

impl UiSettingsWidgetExt for UiBuilder<'_, Entity> {
    fn settings(
        &mut self,
        first_action: SettingsMenuButtonAction,
        second_action: SettingsMenuButtonAction,
        text: &str,
        value: &str,
    ) -> UiBuilder<'_, Entity> {
        self.container(
            (
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
                SettingsWidget,
            ),
            |children| {
                children.text(text, None);
                children.container(
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(30.0),
                            column_gap: Val::Px(5.),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    |children| {
                        children.button_settings_menu("<", first_action);
                        children.text(&value, None).insert(SettingsValues);
                        children.button_settings_menu(">", second_action);
                    },
                );
            },
        )
    }
}
