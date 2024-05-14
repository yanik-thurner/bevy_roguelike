use crate::components::map::Map;
use crate::events::WantsToMoveEvent;
use crate::prelude::*;

pub fn movement_system(mut commands: Commands, mut ev_wants_to_move: EventReader<WantsToMoveEvent>, map: Res<Map>) {
    if ev_wants_to_move.is_empty() {
        return;
    }

    for ev in ev_wants_to_move.read() {
        if map.can_enter_tile(ev.destination) {
            commands.entity(ev.entity).insert(ev.destination);
        }
    }
}