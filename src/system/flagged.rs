use bevy::color::palettes::basic;
use bevy::prelude::*;
use crate::components::*;
use crate::components::uncover::uncover;
use crate::components::flag::flagged;
use crate::resources::board::{Board, FlagToggle};
use crate::resources::events::{TileFlaggedEvent};
use crate::resources::loading::TextureAssets;
use crate::resources::settings::{GameSettings, TileSize};

pub fn flag_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    config: Res<GameSettings>,
    assets: Res<TextureAssets>,
    mut tile_flag_event_rdr: EventReader<TileFlaggedEvent>,
    query: Query<&Children>
) {
    let tile_size = match config.tile_size {
        TileSize::Fixed(size) => size,
        TileSize::Adaptive { .. } => todo!(),
    };
    for event in tile_flag_event_rdr.read() {
        match board.try_toggle_flag(&event.coordinates) {
            FlagToggle::FlagIsSet(e) => {
                info!("Set flag");
                commands.entity(e).with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::from(basic::RED),
                            custom_size:  Some(Vec2::splat(tile_size - config.tile_padding)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 3.),
                        texture: assets.flag.clone(),
                        ..Default::default()
                    });
                });
            }
            FlagToggle::FlagIsUnset(e) => {
                let child = match query.get(e) {
                    Ok(value) => value,
                    Err(e) => continue,
                };
                for c in child {
                    commands.entity(*c).despawn_recursive();
                }
                },
            _ => (),
            }
        }
}