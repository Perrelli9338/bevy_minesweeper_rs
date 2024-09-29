use bevy::prelude::*;
use sickle_ui::{
    prelude::*,
    widgets::layout::tab_container::TabContainer,
};

pub trait TabContainerExt {
    fn add_tab_container(
        &mut self,
        str: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, (Entity, TabContainer)>;
}

impl TabContainerExt for UiBuilder<'_, (Entity, TabContainer)> {
    fn add_tab_container(
        &mut self,
        str: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, (Entity, TabContainer)> {
        self.add_tab(str.into(), |panel| {
            panel
                .container(NodeBundle::default(), spawn_children)
                .style()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Column)
                .row_gap(Val::Px(5.));
        })
    }
}
