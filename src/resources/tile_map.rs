use crate::{
    components::Coordinates,
    resources::tile::Tile,
};
use std::ops::{Deref, DerefMut};
use bevy::utils::HashSet;
use rand::{thread_rng, Rng};

const RANGE: [(i8, i8); 8] = [ // todo!()
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

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item=Coordinates> {
        RANGE.iter().copied().map(move |tuple| coordinates + tuple)
    }

    pub fn get_bomb_tiles(&self) -> impl Iterator<Item=Coordinates> + '_ {
        self.bomb_coordinates.iter().copied()
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        }

        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        let res = self.safe_square_at(coordinates).filter(|c| self.is_bomb_at(*c)).count();
        res as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut r_bombs = bomb_count;
        let mut rng = thread_rng();

        while r_bombs > 0 {
            let row = rng.gen_range(0..self.height) as usize;
            let column = rng.gen_range(0..self.width) as usize;
            if let Tile::Empty | Tile::BombNeighbour(0..=8) = self[row][column] {
                self[row][column] = Tile::Bomb;
                r_bombs -= 1;
            }
            for row in 0..self.height {
                for col in 0..self.width {
                    let coords = Coordinates { y: row, x: col };
                    if self.is_bomb_at(coords) {
                        self.bomb_coordinates.insert(coords);
                        continue;
                    };

                    let bomb_count = self.bomb_count_at(coords);
                    if bomb_count == 0 {
                        continue;
                    }
                    let tile = &mut self[row as usize][col as usize];
                    *tile = Tile::BombNeighbour(bomb_count);
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
