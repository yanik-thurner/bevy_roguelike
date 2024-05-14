use crate::core_logic::map::Map;
use crate::prelude::*;

pub fn random_system(map: Res<Map>, mut query: Query<(Entity, &GridPosition), With<MovingRandomly>>, mut ev_wants_to_move: EventWriter<WantsToMoveEvent>) {
    for (entity, position) in query.iter() {
        let direction = GridPosition::random_direction(false);
        ev_wants_to_move.send(WantsToMoveEvent { entity, destination: position.clone() + direction });
    }
}