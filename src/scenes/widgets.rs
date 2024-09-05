use bevy::prelude::*;
use sickle_ui::ui_builder::UiBuilder;

#[derive(Component)]
struct ButtonWidget;

pub trait UiButtonWidgetExt {
    fn button_widget(&mut self) -> UiBuilder<Entity>;
}

impl UiButtonWidgetExt for UiBuilder<'_, Entity> {
    fn button_widget(&mut self) -> UiBuilder<'_, Entity> {
        self.spawn((NodeBundle::default(), ButtonWidget))
    }
}