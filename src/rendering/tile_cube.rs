use crate::{resources::tile::Tile,
            components::coordinates::FaceSideIndex};

use std::ops::{Add, Deref, DerefMut};
use bevy::prelude::Component;
use bevy::utils::HashSet;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct TileCube {
    bomb_count: u16,
    map: Vec<Tile>
}


impl TileCube {
    pub fn safe_square_at(&self, index: u16) -> impl Iterator<Item=u16> {
        match index {
            _ => [0, 0, 0, 0]
        }.iter().map(|n| n as u16).collect()
    }

    pub fn new() -> Self {
        let map = vec![Tile::Empty; 6];
        Self {
            bomb_count: 5,
            map,
        }
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut r_bombs = bomb_count;
        let mut rng = thread_rng();

        while r_bombs > 0 {
            let i = rng.gen_range(0..=6) as usize;
            if let Tile::Empty | Tile::BombNeighbour(0..=8) = self[i] {
                self[i] = Tile::Bomb;
                r_bombs -= 1;
            }
                for index in 0..=6{
                    if self.is_bomb_at(index) {
                        continue;
                    };

                    let bomb_count = self.bomb_count_at(index);
                    if bomb_count == 0 {
                        continue;
                    }
                    let tile = &mut self[index];
                    *tile = Tile::BombNeighbour(bomb_count);
            }
        }
    }

    pub fn bomb_count_at(&self, index: usize) -> u8 {
        if self.is_bomb_at(index) {
            return 0;
        }

        let res = self.safe_square_at(index).filter(|c| self.is_bomb_at(*c)).count();
        res as u8
    }

    pub fn is_bomb_at(&self, index: usize) -> bool {
        self.map[index].is_bomb()
    }


    pub fn get_bomb_count(&self) -> u16 {
        self.bomb_count
    }

}

impl Deref for TileCube {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileCube {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
