use bevy::ecs::component::Component;
use std::fmt::format;
use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Tile {
    Bomb,
    BombNeighbour(u8),
    Flag,
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }
}
