use crate::prelude::*;

pub fn random_system(movers_query: Query<(Entity, &GridPosition), With<MovingRandomly>>, target_query: Query<(Entity, &GridPosition), With<Health>>, mut ev_wants_to_move: EventWriter<WantsToMoveEvent>, mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>) {
    for (current_mover_entity, current_mover_position) in movers_query.iter() {
        let direction = GridPosition::random_direction(false);

        let target = target_query
            .iter()
            .filter(|(potential_target, potential_target_position)| **potential_target_position == (*current_mover_position + direction) && *potential_target != current_mover_entity)
            .map(|(target, _)| target)
            .next();


        if let Some(enemy) = target {
            ev_wants_to_attack.send(WantsToAttackEvent { attacker: current_mover_entity, victim: enemy });
        } else {
            ev_wants_to_move.send(WantsToMoveEvent { entity: current_mover_entity, destination: *current_mover_position + direction });
        }
    }
}