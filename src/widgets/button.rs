use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::{
    components::uisettings::UISettings,
    scenes::{
        MenuButtonAction,
        settings_menu_plugin::SettingsMenuButtonAction
    },
    widgets::text::UiTextWidgetExt
};

#[derive(Component)]
struct ButtonWidget;

pub trait UiButtonWidgetExt {
    fn button_MainMenu(&mut self, text: &str, action: MenuButtonAction);
    fn button_SettingsMenu(&mut self, text: &str, action: SettingsMenuButtonAction);
}

impl UiButtonWidgetExt for UiBuilder<'_, Entity> {
    fn button_SettingsMenu(&mut self, text: &str, action: SettingsMenuButtonAction) {
        let settings = UISettings::default();
        self
            .container((
                           ButtonBundle {
                               style: settings.button_settings_style,
                               background_color: settings.button_colors.normal.into(),
                               border_radius: settings.button_border_style,
                               ..Default::default()
                           },
                           settings.button_colors,
                           action,
                       ), |children| {
                children.text(text);
            }).insert(ButtonWidget);
    }

    fn button_MainMenu(&mut self, text: &str, action: MenuButtonAction) {
        let settings = UISettings::default();
        self
            .container((
                           ButtonBundle {
                               style: settings.button_style,
                               background_color: settings.button_colors.normal.into(),
                               border_radius: settings.button_border_style,
                               ..Default::default()
                           },
                           settings.button_colors,
                           action,
                       ), |children| {
                children.text(text);
            }).insert(ButtonWidget);
    }
}
