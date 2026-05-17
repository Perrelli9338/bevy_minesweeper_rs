use crate::{
    components::{
        Coordinates
    },
    game::{
        board::Board,
        bounds::Bounds2,
        settings::{GameSettings, Position, TileSize},
        tile_map::TileMap
    },
    resources::{
        assets::{
            FontAssets, TextureAssets
        },
        GameState
    },
    AppState
};
use bevy::{
    color::palettes::basic,
    prelude::*
};
use std::collections::HashMap;
use crate::components::{Bomb, BombNeighbor};
use crate::game::events::TileTriggerEvent;
use crate::game::tile::Tile;

pub mod board;
pub mod bounds;
pub mod events;
pub mod settings;
pub mod tile;
pub mod tile_map;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), Self::create)
            .add_systems(Update, Self::init_board.run_if(in_state(GameState::FirstMove)))
            .add_systems(
                Update,
                new_game
                    .run_if(in_state(GameState::Disabled))
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn new_game(
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::FirstMove);
}

impl BoardPlugin {
    pub fn create(
        mut commands: Commands,
        options: Res<GameSettings>,
        assets: (Res<TextureAssets>, Res<FontAssets>),
    ) {
        let (textures, fonts) = assets;
        let config = options.clone();

        let tile_size = match config.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => 50.0,
        };

        let mut tile_map = TileMap::new(config.map_size.0, config.map_size.1);
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.get_width() * tile_map.get_height()).into());

        let board_size = Vec2::new(
            tile_map.get_width() as f32 * tile_size,
            tile_map.get_height() as f32 * tile_size,
        );

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

        let e = commands
            .spawn((
                Name::new("Board"),
                SpatialBundle {
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                Self::generate(
                    parent,
                    &tile_map,
                    tile_size,
                    config.tile_padding,
                    Color::WHITE,
                    textures.tile.clone(),
                    Color::from(basic::TEAL),
                    &mut covered_tiles,
                );
            })
            .id();

        commands.insert_resource(Board {
            tile_map: tile_map.clone(),
            bounds: Bounds2 {
                position: position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            flagged_tiles: Default::default(),
            entity: e,
            safe_start: config.easy_mode
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn generate(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        background_color: Color,
        tile_image: Handle<Image>,
        covered_background_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
    ) {
        let size = tile_size - tile_padding;
        let sprites_size = Some(Vec2::splat(size));
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut commands = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: background_color,
                        custom_size: sprites_size,
                        ..Default::default()
                    },

                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    texture: tile_image.clone(),
                    ..Default::default()
                });

                commands.insert(coordinates);

                commands.with_children(|parent| {
                    let e = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: sprites_size,
                                color: covered_background_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            texture: tile_image.clone(),
                            ..Default::default()
                        })
                        .id();
                    covered_tiles.insert(coordinates, e);
                });


            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn init_board(
        mut commands: Commands,
        mut board: ResMut<Board>,
        config: Res<GameSettings>,
        assets: Res<TextureAssets>,
        fonts: Res<FontAssets>,
        mut tile_trigger_evr: EventReader<TileTriggerEvent>,
        covered_query: Query<&Parent>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        let start_event = tile_trigger_evr.read().next();
        if let Some(event) = start_event {
            let safe_click = event.coordinates;
            let coord = if board.safe_start {
                Some(safe_click)
            } else {
                None
            };
            board.tile_map.set_bombs(config.bomb_count, coord);

            let tile_size = match config.tile_size {
                TileSize::Fixed(v) => v,
                TileSize::Adaptive { .. } => 50.0,
            } - config.tile_padding;
            let sprites_size = Some(Vec2::splat(tile_size));

            for (y, line) in board.tile_map.iter().enumerate() {
                for (x, tile) in line.iter().enumerate() {
                    let coords = Coordinates { x: x as u16, y: y as u16 };

                    if let Some(covered_entity) = board.covered_tiles.get(&coords) {
                        if let Ok(tile_parent) = covered_query.get(*covered_entity) {
                            let tile_entity = tile_parent.get();

                            match tile {
                                Tile::Bomb => {
                                    commands.entity(tile_entity).insert(Bomb);
                                    commands.entity(tile_entity).with_children(|parent| {
                                        parent.spawn(SpriteBundle {
                                            sprite: Sprite {
                                                custom_size: sprites_size,
                                                color: Color::from(bevy::color::palettes::basic::RED),
                                                ..Default::default()
                                            },
                                            transform: Transform::from_xyz(0., 0., 1.),
                                            texture: assets.bomb.clone(),
                                            ..Default::default()
                                        });
                                    });
                                }
                                Tile::BombNeighbour(count) => {
                                    commands.entity(tile_entity).insert(BombNeighbor { count: *count });
                                    commands.entity(tile_entity).with_children(|parent| {
                                        parent.spawn(bomb_count_text_bundle(
                                            *count,
                                            fonts.font.clone(),
                                            tile_size,
                                        ));
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            game_state.set(GameState::Playing);
        }
    }
}

pub fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
    let color = match count {
        1 => Color::from(basic::BLUE),
        2 => Color::from(basic::GREEN),
        3 => Color::from(basic::RED),
        4 => Color::from(basic::NAVY),
        5 => Color::from(basic::MAROON),
        6 => Color::from(basic::AQUA),
        7 => Color::from(basic::PURPLE),
        _ => Color::from(basic::SILVER),
    };

    let style = TextStyle {
        font,
        font_size,
        color,
    };
    // adopted 0.9 to 0.10 and simplified API
    let text = Text::from_section(count.to_string(), style).with_no_wrap();

    Text2dBundle {
        text,
        // z-order, print text on top of the tile
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    }
}
