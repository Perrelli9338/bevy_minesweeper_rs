use bevy::ecs::event::Event;

use crate::components::Coordinates;

#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent{
    pub coordinates: Coordinates
}

#[derive(Debug, Clone, Copy, Event)]
pub struct TileFlaggedEvent{
    pub coordinates: Coordinates
}

#[derive(Debug, Clone, Copy, Event)]
pub struct GameWin;

#[derive(Debug, Clone, Copy, Event)]
pub struct GameLose;

