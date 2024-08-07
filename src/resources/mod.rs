use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::actions::{Actions, set_movement_actions};
use crate::components::Coordinates;
use crate::GameState;
use crate::resources::{settings::{TileSize, TileSize::*, GameSettings},
                       board::TileMap};
use crate::resources::settings::Position;


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

    pub fn create(mut commands: Commands, options: Option<Res<GameSettings>>) {
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
                tile_map.iter().enumarete().map(
                    |(y, line)| line.iter().enumerate().map(
                        |x, tile| parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLACK,
                                custom_size: Some(Vec2::splat(tile_size - config.tile_padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                tile_size / 2.,
                                tile_size / 2.,
                                1.,
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new(format!("{}, {}", x, y)))
                        .insert(Coordinates { x, y })
                    ));
            });

    }
    pub fn new() {
        let mut tile_map = TileMap::new(20, 20);
        tile_map.set_bombs(40);
        #[cfg(debug_assertions)]
        log::info!("{}", tile_map.log());
    }
}