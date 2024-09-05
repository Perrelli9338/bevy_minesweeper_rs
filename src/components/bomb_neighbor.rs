use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Component)]
pub struct BombNeighbor {
    pub count: u8,
}