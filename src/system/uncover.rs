use bevy::prelude::*;
use crate::components::*;
use crate::components::uncover::uncover;
use crate::components::flag::flagged;
use crate::resources::board::Board;
use crate::resources::events::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>
) {
    for trigger_event in tile_trigger_evr.read() {
        if let Some(entity) = board.tile_selected(&trigger_event.coordinates) { // todo(Bug: Double left click uncover when shouldn't)!
            commands.entity(*entity).insert(uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<uncover>>,
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
            None => log::info!("Tried to uncover an already uncovered tile"),
            Some(e) => log::info!("Uncovered tile {} (entity: {:?})",coordinates, e),
        }
        
        if bomb.is_some() {
            log::info!("Boom!");
        }
        else if bomb_counter.is_none() {
            for entity in board.uncover_tile_neighbour(*coordinates) {
                commands.entity(entity).insert(uncover);
            }
        }
    } 
}