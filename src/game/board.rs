use crate::{components::Coordinates, game::{
    bounds::Bounds2,
    TileMap
}
};
use bevy::{ecs::system::Resource, math::Vec2, prelude::*};
use std::collections::{HashMap, HashSet};

pub enum FlagToggle {
    FlagIsSet(Entity),
    FlagIsUnset(Entity),
    Nothing,
}

#[derive(Debug, Clone, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub flagged_tiles: HashSet<Coordinates>,
    pub entity: Entity,
}

impl Board {
    pub fn press_position(&self, camera: &Camera, transform: &GlobalTransform, position: Vec2) -> Option<Coordinates> {
        if let Some(position_cursor) = camera.viewport_to_world_2d(transform, position) {
            if !self.bounds.in_bounds(position_cursor) {
                return None;
            }

            let coordinates = position_cursor - self.bounds.position;
            Some(Coordinates {
                x: (coordinates.x / self.tile_size) as u16,
                y: (coordinates.y / self.tile_size) as u16,
            })
        } else { None }
    }

    pub fn tile_selected(&self, coordinates: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coordinates)
    }

    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        if self.flagged_tiles.contains(coordinates) {
            None
        } else {
            self.covered_tiles.remove(coordinates)
        }
    }

    pub fn try_toggle_flag(&mut self, coordinates: &Coordinates) -> FlagToggle {
        match self.covered_tiles.get(coordinates) {
            Some(e) => {
                if self.flagged_tiles.contains(coordinates) {
                    self.flagged_tiles.remove(coordinates);
                    FlagToggle::FlagIsUnset(*e)
                } else {
                    self.flagged_tiles.insert(*coordinates);
                    FlagToggle::FlagIsSet(*e)
                }
            }
            _ => FlagToggle::Nothing,
        }
    }

    pub fn is_win(&self, flag_mode: bool) -> bool {
        if flag_mode {
            self.tile_map.get_bomb_count() as usize == self.flagged_tiles.len()
                && self.tile_map.get_bomb_count() as usize == self.covered_tiles.len()
        } else {
            self.tile_map.get_bomb_count() as usize == self.covered_tiles.len()
        }
    }

    pub fn uncover_tile_neighbour(&self, coordinate: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coordinate)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    pub fn uncover_bomb(&self) -> Vec<Entity> {
        self.tile_map
            .get_bomb_tiles()
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}
