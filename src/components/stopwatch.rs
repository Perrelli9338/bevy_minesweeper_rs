use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
struct GameStopwatch(u16);
