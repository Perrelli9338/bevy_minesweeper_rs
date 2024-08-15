use std::collections::{HashMap, HashSet};

use crate::{
    resources::bounds::Bounds2,
    components::Coordinates, resources::TileMap,
};

use bevy::{
    ecs::system::Resource,
    math::Vec2, prelude::*,
    window::Window
};

pub(crate) enum FlagToggle {
    FlagIsSet(Entity),
    FlagIsUnset(Entity),
    Nothing,
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub flagged_tiles: HashSet<Coordinates>,
    pub entity: Entity,
}

impl Board {
    pub(crate) fn press_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let windows_size = Vec2 {
            x: window.width(),
            y: window.height(),
        };

        let position_cursor = position - (windows_size / 2.0);
        if !self.bounds.in_bounds(position_cursor) {
            return None;
        }

        let coordinates = position_cursor - self.bounds.position;
        return Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: self.tile_map.get_height() - 1 - (coordinates.y / self.tile_size) as u16,
        })
    }

    pub fn tile_selected(&self, coordinates: &Coordinates) -> Option<&Entity> {
        return self.covered_tiles.get(coordinates);
    }

    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        return if self.flagged_tiles.contains(coordinates) {
            None
        } else {
            self.covered_tiles.remove(coordinates)
        }
    }

    pub fn try_toggle_flag(&mut self, coordinates: &Coordinates) -> FlagToggle {
        return match self.covered_tiles.get(coordinates) {
            Some(e) => {
                if self.flagged_tiles.contains(coordinates) {
                    self.flagged_tiles.remove(coordinates);
                    FlagToggle::FlagIsUnset(e.clone())
                } else {
                    self.flagged_tiles.insert(*coordinates);
                    FlagToggle::FlagIsSet(e.clone())
                }
            }
            _ => FlagToggle::Nothing,
        }
    }
    
    pub fn is_win(&self) -> bool {
        return self.tile_map.get_bomb_count() as usize == self.flagged_tiles.len() && self.tile_map.get_bomb_count() as usize == self.covered_tiles.len()
    }

    pub fn uncover_tile_neighbour(&self, coordinate: Coordinates) -> Vec<Entity> {
        return self
            .tile_map
            .safe_square_at(coordinate)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect();
    }

    pub fn uncover_bomb(&self, coordinate: Coordinates) -> Vec<Entity> {
        return self
            .tile_map
            .get_bomb_tiles()
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect();
    }
}