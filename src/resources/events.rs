use bevy::ecs::event::Event;
use crate::components::Coordinates;

#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent {
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct TileFlaggedEvent {
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct GameWinEvent;

#[derive(Debug, Clone, Copy, Event)]
pub struct GameLoseEvent;

#[derive(Debug, Clone, Copy, Event)]
pub struct EndgameEvent;

