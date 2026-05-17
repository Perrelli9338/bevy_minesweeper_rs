use bevy::prelude::*;
use web_time::{Duration, Instant};

#[derive(Resource)]
pub(crate) struct GameTimer {
    pub start_time: Instant,
    pub duration: Duration,
}

impl GameTimer {
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            start_time: Instant::now(),
            duration: Duration::from_secs_f32(seconds),
        }
    }

    pub fn finished(&self) -> bool {
        Instant::now().duration_since(self.start_time) >= self.duration
    }
}
