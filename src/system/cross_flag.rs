use bevy::{
    color::palettes::basic,
    prelude::*,
};
use crate::{
    components::{
        Bomb,
        flag::Flagged,
    },
    resources::{
        assets::TextureAssets,
        events::GameLoseEvent,
        settings::{GameSettings, TileSize},
    },
};

pub fn uncover_wrong_flags(
    mut commands: Commands,
    children: Query<Entity, (With<Flagged>, Without<Bomb>)>,
    query: Query<&Children>,
    config: Res<GameSettings>,
    assets: Res<TextureAssets>,
    mut lose_evr: EventReader<GameLoseEvent>,
) {
    for _e in lose_evr.read() {
        let tile_size = match config.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => todo!(),
        };
        for entity in children.iter() {
            let child = match query.get(entity) {
                Ok(value) => value,
                Err(_e) => continue,
            };
            for c in child {
                commands.entity(*c).despawn_recursive();
            }
            commands.entity(entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(basic::RED),
                        custom_size: Some(Vec2::splat(tile_size - config.tile_padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., 3.),
                    texture: assets.wrong.clone(),
                    ..Default::default()
                });
            });
        }
    }
}