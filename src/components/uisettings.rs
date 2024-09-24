use bevy::prelude::{AlignItems, BorderRadius, Component, JustifyContent, Style, Val};
use crate::components::button_colors::ButtonColors;

#[derive(Component, Clone)]
pub(crate) struct UISettings {
    pub(crate) button_colors: ButtonColors,
    pub(crate) button_style: Style,
    pub(crate) button_border_style: BorderRadius,
    pub(crate) button_settings_style: Style,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            button_colors: ButtonColors::default(),
            button_style: Style {
                width: Val::Px(140.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            button_settings_style: Style {
                width: Val::Px(25.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            button_border_style: BorderRadius::all(
                Val::Px(8.),
            ),
        }
    }
}
