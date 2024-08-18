use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct GameStopwatch {
    pub(crate) time: Stopwatch
}

impl GameStopwatch {

    pub(crate) fn new(mut commands: Commands) {
        commands.insert_resource(GameStopwatch {
            time: Stopwatch::new(),
        });
    }
    pub(crate) fn tick(
        time: Res<Time>,
        mut stopwatch: ResMut<GameStopwatch>
    ){
        stopwatch.time.tick(time.delta());
    }

    pub(crate) fn unpause(
        mut stopwatch: ResMut<GameStopwatch>
    ){
        stopwatch.time.unpause();
    }

    pub(crate) fn pause(
        mut stopwatch: ResMut<GameStopwatch>
    ){
        stopwatch.time.paused();
    }
}
