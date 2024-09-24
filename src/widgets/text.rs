use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
struct TextWidget;

pub trait UiTextWidgetExt {
    fn text(&mut self, text: &str) -> UiBuilder<'_, Entity>;
}
impl UiTextWidgetExt for UiBuilder<'_, Entity> {
    fn text(&mut self, text: &str) -> UiBuilder<'_, Entity> {
        self.spawn((TextBundle::from_section(text, TextStyle::default()), TextWidget))
        /*self.spawn((TextBundle::from_section(
            text,
            TextStyle {
                ..default()
            },
            TextWidget
        )), TextWidget)*/
    }
}