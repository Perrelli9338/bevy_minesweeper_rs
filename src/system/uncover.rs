use bevy::prelude::*;
use crate::components::*;
use crate::components::uncover::Uncover;
use crate::components::flag::flagged;
use crate::resources::board::Board;
use crate::resources::events::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>
) {
    for e in tile_trigger_evr.read() {
        if !board.flagged_tiles.contains(&e.coordinates) {
            if let Some(e) = board.tile_selected(&e.coordinates) {
                commands.entity(*e).insert(Uncover);
            }
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();
        let (coordinates, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                continue;
            }
        };
        match board.try_uncover_tile(coordinates) {
            Some(_) => {
                if bomb.is_some() {
                    for entity in board.uncover_bomb(*coordinates) {
                        commands.entity(entity).insert(Uncover);
                    }
                }
                else if bomb_counter.is_none() {
                    for entity in board.uncover_tile_neighbour(*coordinates) {
                        commands.entity(entity).insert(Uncover);
                    }
                }
            }
            _ => {}
        }
        
        
    } 
}