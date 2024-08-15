use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct GameTimer(pub Timer);
