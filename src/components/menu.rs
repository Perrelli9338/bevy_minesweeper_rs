use crate::resources::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
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

pub struct MenuPlugin;

// This plugin is responsible for the game menu (containing only one button...)
// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
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

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    let round_corner = 8.;
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(15.),
                        padding: UiRect::all(Val::Px(15.)),
                        ..default()
                    },
                    ..default()
                },
                Menu,
            ))
            .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Minesweeper",
                        TextStyle {
                            font_size: 64.,
                            ..default()
                        }
                    ));
                    children.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(64.),
                            ..default()
                        },
                        ..default()
                    });
            });
        })
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        border_radius: BorderRadius::new(
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                        ),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            ..default()
                        }
                    ));
                });
        })
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        border_radius: BorderRadius::new(
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                            Val::Px(round_corner),
                        ),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Close),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            ..default()
                        }
                    ));
                });
        });
}

fn click_play_button(
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
