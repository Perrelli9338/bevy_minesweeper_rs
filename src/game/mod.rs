use crate::{
    components::{
        timer::GameTimer,
        uncover::Uncover,
        Bomb, BombNeighbor, Coordinates
    },
    game::{
        board::Board,
        bounds::Bounds2,
        settings::{GameSettings, Position, TileSize},
        tile::Tile,
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
use std::collections::{HashMap, HashSet};

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
            .add_systems(
                Update,
                new_game
                    .run_if(in_state(GameState::Disabled))
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn new_game(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Playing);
    }
}

impl BoardPlugin {
    pub fn create(
        mut commands: Commands,
        options: Res<GameSettings>,
        assets: (Res<TextureAssets>, Res<FontAssets>),
    ) {
        let mut safe_start: Option<Entity> = None;

        let (textures, fonts) = assets;
        let config = options.clone();

        let tile_size = match config.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => todo!(),
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

        tile_map.set_bombs(config.bomb_count);

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
                    fonts.font.clone(),
                    Color::WHITE,
                    textures.bomb.clone(),
                    textures.tile.clone(),
                    textures.covered_tile.clone(),
                    Color::from(basic::TEAL),
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();

        if config.easy_mode {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map: tile_map.clone(),
            bounds: Bounds2 {
                position: position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            flagged_tiles: HashSet::new(),
            entity: e,
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn generate(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        font: Handle<Font>,
        background_color: Color,
        bomb_image: Handle<Image>,
        tile_image: Handle<Image>,
        covered_tile_image: Handle<Image>,
        covered_background_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start: &mut Option<Entity>,
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
                            texture: covered_tile_image.clone(),
                            ..Default::default()
                        })
                        .id();
                    covered_tiles.insert(coordinates, e);
                    if safe_start.is_none() && *tile == Tile::Empty || !(*tile == Tile::Bomb) {
                        *safe_start = Some(e);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        commands.insert(Bomb);
                        commands.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::from(basic::RED),
                                    custom_size: sprites_size,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbour(bombs_count) => {
                        commands.insert(BombNeighbor {
                            count: *bombs_count,
                        });
                        commands.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *bombs_count,
                                font.clone(),
                                size,
                            ));
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
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
}
