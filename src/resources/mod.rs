use bevy::a11y::accesskit::TextAlign;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use crate::actions::{Actions, set_movement_actions};
use crate::components::*;
use crate::{components, GameState};
use crate::resources::{settings::{TileSize, TileSize::*, GameSettings},
                       board::TileMap};
use crate::resources::settings::Position;
use crate::resources::loading::TextureAssets;
use crate::resources::tile::Tile;


pub(crate) mod tile;
pub(crate) mod audio;
pub(crate) mod loading;
pub(crate) mod board;
pub(crate) mod settings;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App){
        #[cfg(feature = "debug")]
        app.add_plugins(WorldInspectorPlugin::new());
        app.insert_resource(GameSettings {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.0,
            tile_size: Fixed(20.0),
            ..Default::default()
        })
        .add_systems(OnEnter(GameState::Playing), Self::create);
    }

}

impl BoardPlugin {

    pub fn create(mut commands: Commands, options: Option<Res<GameSettings>>, textures: Res<TextureAssets>) {
        let config = match options {
            None => GameSettings::default(),
            Some(c) => c.clone(),
        };

        let tile_size = match config.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => todo!(),
        };

        let mut tile_map = TileMap::new(config.map_size.0, config.map_size.1);

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.width() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let position = match config.position {
            Position::Centered { offset } => {
                Vec3 {
                    x: -(board_size.x / 2.0),
                    y: -(board_size.y / 2.0),
                    z: 0.0,
                } + offset
            }
            Position::Custom(p) => p,
        };

        tile_map.set_bombs(config.bomb_count);

        commands.spawn((
            Name::new("Board"),
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        ))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..Default::default()
                })
                    .insert(Name::new("Background"));

                Self::generate(
                    parent,
                    tile_map,
                    tile_size,
                    config.tile_padding,
                    Color::linear_rgb(0., 0., 255.),
                    textures.font.clone(),
                    textures.bomb.clone()
                );
            });
    }
    fn generate(
        parent: &mut ChildBuilder,
        tile_map: TileMap,
        tile_size: f32,
        tile_padding: f32,
        background_color: Color,
        font: Handle<Font>,
        bomb_image: Handle<Image>
    ){
        for (y , line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
            let mut commands = parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: background_color,
                    custom_size: Some(Vec2::splat(tile_size - tile_padding as f32)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * tile_size) + (tile_size / 2.),
                    (y as f32 * tile_size) + (tile_size / 2.),
                    1.,
                ),
                ..Default::default()
            });

                match tile {
                    Tile::Bomb => {
                        commands.insert(components::Bomb);
                        commands.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbour(bombs_count) => {
                        commands.insert(components::BombNeighbor{count: *bombs_count});
                        commands.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *bombs_count,
                                font.clone(),
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                    _ => (),
                }
                }
            }
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
        let color = match count {
            _ => Color::BLACK,
        };

        let style = TextStyle {
            font,
            font_size,
            color,
        };
        // adopted 0.9 to 0.10 and simplified API
        let text =
            Text::from_section(count.to_string(), style).with_justify(JustifyText::Center);

        Text2dBundle {
            text,
            // z-order, print text on top of the tile
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        }
    }

    pub fn new() {
        let mut tile_map = TileMap::new(20, 20);
        tile_map.set_bombs(40);
        #[cfg(debug_assertions)]
        log::info!("{}", tile_map.log());
    }
}