use crate::prelude::*;

pub fn random_system(movers_query: Query<(Entity, &Position), With<MovingRandomly>>, target_query: Query<(Entity, &Position), With<Health>>, mut ev_wants_to_move: EventWriter<WantsToMoveEvent>, mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>, mut rng: Query<&mut EntropyComponent<WyRand>, With<RngSource>>) {
    for (current_mover_entity, current_mover_position) in movers_query.iter() {
        let direction = GridDirection::ALL_DIRECTIONS[RngHelper::generate_from_range(rng.get_single_mut().unwrap().next_u32(), 0..9) as usize];

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