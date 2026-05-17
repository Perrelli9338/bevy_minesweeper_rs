use std::time::Duration;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::winit::{UpdateMode, WinitSettings};
use Minesweeper::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1.0 / 120.0)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f64(1.0 / 120.0)),
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
