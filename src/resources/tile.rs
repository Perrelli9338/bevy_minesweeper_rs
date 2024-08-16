use bevy::ecs::component::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Tile {
    Bomb,
    BombNeighbour(u8),
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }
}
