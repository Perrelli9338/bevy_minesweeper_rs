use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    pub(crate) normal: Color,
    pub(crate) hovered: Color,
    pub(crate) pressed: Color,
    pub(crate) disabled: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
            pressed: Color::linear_rgb(0.5, 0.5, 0.5),
            disabled: Color::linear_rgb(0.35, 0.35, 0.35),
        }
    }
}