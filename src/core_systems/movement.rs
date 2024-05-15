use crate::components::map::Map;
use crate::events::WantsToMoveEvent;
use crate::prelude::*;

pub fn movement_system(mut commands: Commands, mut ev_wants_to_move: EventReader<WantsToMoveEvent>, query: Query<&Player>, mut next_state: ResMut<NextState<TurnState>>, map: Res<Map>) {
    if ev_wants_to_move.is_empty() {
        return;
    }

    let mut end_turn = false;

    for ev in ev_wants_to_move.read() {
        if map.can_enter_tile(ev.destination) {
            commands.entity(ev.entity).insert(ev.destination);
            if query.contains(ev.entity) {
                end_turn = true;
            }
        }
    }

    if end_turn {
        next_state.set(TurnState::PlayerTurn);
    }
}