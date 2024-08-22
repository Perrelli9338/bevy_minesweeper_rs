use std::time::{Duration, Instant};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameStopwatch {
    pub(crate) time: Instant,
    pub(crate) total_time: Duration,
}

impl GameStopwatch {

    pub(crate) fn new(mut commands: Commands) {
        commands.insert_resource(GameStopwatch {
            time: Instant::now(),
            total_time: Duration::ZERO,
        });
    }

    pub(crate) fn pause(
        mut stopwatch: ResMut<GameStopwatch>
    ){
        stopwatch.total_time = Instant::now() - stopwatch.time;
    }
}
