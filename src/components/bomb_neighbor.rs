use bevy::prelude::Component;

#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct BombNeighbor {
    pub count: u8
}