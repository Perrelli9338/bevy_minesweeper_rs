mod main_menu_plugin;
mod settings_menu_plugin;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameState;
use crate::resources::loading::TextureAssets;



#[derive(Component, Clone)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

#[derive(Component)]
struct Menu;

#[derive(Component)]
pub struct UISettings {
    pub round_corner: f32,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            round_corner: 8.,
        }
    }
}

pub struct MenuPlugin;

// This plugin is responsible for the game menu (containing only one button...)
// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            main_menu_plugin::main_menu,
            settings_menu_plugin::settings_menu
        ))
            .add_systems(Startup, set_camera);
    }
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

fn set_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn button_states(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
