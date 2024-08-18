use bevy::{prelude::Vec3, ecs::system::Resource};
use serde::{Deserialize, Serialize};
use crate::resources::settings::TileSize::Fixed;

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position{
    Centered { offset: Vec3 },
    Custom(Vec3),
}

/// Board generation options.
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GameSettings {
    pub map_size: (u16, u16),
    pub bomb_count: u16,
    pub position: Position,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub easy_mode: bool,
    pub timer_start: f32,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            map_size: (8, 8),
            bomb_count: 10,
            tile_padding: 3.0,
            tile_size: Fixed(50.0),
            easy_mode: true,
            position: Default::default(),
            timer_start: 0.8,
        }
    }
}