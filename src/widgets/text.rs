use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
struct TextWidget;

pub trait UiTextWidgetExt {
    fn text(&mut self, text: &str, size: Option<f64>) -> UiBuilder<'_, Entity>;
}
impl UiTextWidgetExt for UiBuilder<'_, Entity> {
    fn text(&mut self, text: &str, size: Option<f64>) -> UiBuilder<'_, Entity> {
        let font_size = size.unwrap_or(21.);
        self.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: font_size as f32,
                    ..default()
                },
            ),
            TextWidget,
        ))
    }
}
