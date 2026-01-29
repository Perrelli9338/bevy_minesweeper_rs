use std::iter::once;
use crate::{components::Coordinates, game::tile::Tile};
use bevy::utils::HashSet;
use rand::{thread_rng};
use std::ops::{Deref, DerefMut};
use rand::seq::SliceRandom;

const RANGE: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_coordinates: HashSet<Coordinates>,
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new(width: u16, height: u16) -> Self {
        let map = vec![vec![Tile::Empty; width as usize]; height as usize];
        Self {
            bomb_coordinates: HashSet::new(),
            bomb_count: 9,
            height,
            width,
            map,
        }
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        RANGE.iter().copied().map(move |tuple| coordinates + tuple)
    }

    pub fn get_bomb_tiles(&self) -> impl Iterator<Item = Coordinates> + '_ {
        self.bomb_coordinates.iter().copied()
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        !(coordinates.x >= self.width || coordinates.y >= self.height) &&
            self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    fn coord_to_idx(&self, c: Coordinates) -> usize {
        (c.y as usize * self.width as usize) + c.x as usize
    }

    fn idx_to_coord(&self, idx: usize) -> Coordinates {
        Coordinates {
            x: (idx % self.width as usize) as u16,
            y: (idx / self.width as usize) as u16,
        }
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }
        let res = self
            .safe_square_at(coordinates)
            .filter(|c| self.is_bomb_at(*c))
            .count();
        res as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16, safe_coord: Option<Coordinates>) {
        self.bomb_count = bomb_count;
        let mut rng = thread_rng();
        let total_tiles = self.width as usize * self.height as usize;
        let mut tiles: Vec<usize> = (0..total_tiles).collect();

        let mut tiles_range = HashSet::new();

        if let Some(coord) = safe_coord {
            let neighbours: Vec<usize> = self.safe_square_at(coord)
                .chain(once(coord))
                .map(|c| self.coord_to_idx(c))
                .collect();

            if total_tiles.saturating_sub(neighbours.len()) >= bomb_count as usize {
                tiles_range.extend(neighbours);
            } else {
                tiles_range.insert(self.coord_to_idx(coord));
            }
        }

        tiles.retain(|idx| !tiles_range.contains(idx));

        tiles.shuffle(&mut rng);

        let bombs = tiles.len().min(bomb_count as usize);

        for &idx in tiles.iter().take(bombs) {
            let coord = self.idx_to_coord(idx);

            self[coord.y as usize][coord.x as usize] = Tile::Bomb;
            self.bomb_coordinates.insert(coord);
        }

        for bomb_tile in self.bomb_coordinates.iter().cloned().collect::<Vec<_>>() {
            for neighbor in self.safe_square_at(bomb_tile) {
                if neighbor.x >= self.width || neighbor.y >= self.height {
                    continue;
                }

                let tile = &mut self[neighbor.y as usize][neighbor.x as usize];
                match tile {
                    Tile::Empty => *tile = Tile::BombNeighbour(1),
                    Tile::BombNeighbour(count) => *tile = Tile::BombNeighbour(*count + 1),
                    _ => {}
                }
            }
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_bomb_count(&self) -> u16 {
        self.bomb_count
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
