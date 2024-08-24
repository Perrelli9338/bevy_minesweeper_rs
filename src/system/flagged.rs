use bevy::{
    color::palettes::basic,
    prelude::*
};
use crate::components::flag::Flagged;
use crate::components::uncover::Uncover;
use crate::resources::{
    board::{Board, FlagToggle},
    events::{GameWinEvent, TileFlaggedEvent},
    assets::TextureAssets,
    settings::{GameSettings, TileSize},
};

pub fn flag_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    config: Res<GameSettings>,
    assets: Res<TextureAssets>,
    mut tile_flag_event_rdr: EventReader<TileFlaggedEvent>,
    mut trigger_event: EventWriter<GameWinEvent>,
    query: Query<&Children>
) {
    let tile_size = match config.tile_size {
        TileSize::Fixed(size) => size,
        TileSize::Adaptive { .. } => todo!(),
    };
    for event in tile_flag_event_rdr.read() {
        match board.try_toggle_flag(&event.coordinates) {
            FlagToggle::FlagIsSet(e) => {
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
                }).try_insert(Flagged);
            }
            FlagToggle::FlagIsUnset(e) => {
                let child = match query.get(e) {
                    Ok(value) => value,
                    Err(_e) => continue,
                };
                for c in child {
                    commands.entity(*c).despawn_recursive();
                    commands.entity(e).remove::<Flagged>();
                    commands.entity(e).remove::<Uncover>();
                }
                },
            _ => (),
            }
        }
    if board.is_win(config.flag_mode){
        trigger_event.send(GameWinEvent);
    }
}