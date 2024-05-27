use crate::prelude::*;

pub fn combat_system(mut commands: Commands, mut ev_attack: EventReader<WantsToAttackEvent>, mut combat_query: Query<(&mut Health, Entity)>, player_query: Query<Entity, With<Player>>, state: Res<State<TurnState>>) {
    let player = player_query.get_single().unwrap();

    let attacker_victim_pairs: Vec<_> = ev_attack.read()
        .filter(|event| commands.get_entity(event.attacker).is_some() && commands.get_entity(event.victim).is_some())
        .filter(|event| !(event.attacker == player && *state == TurnState::WorldTurn))
        .map(|event| (event.attacker, event.victim, event.direction)).collect();


    attacker_victim_pairs.iter().for_each(|(attacker, victim, direction)| {
        println!("Combat: {:?} attacks {:?} in {:?}", attacker, victim, state.get());
        commands.entity(*attacker).insert(Animation::new_attack_animation(*direction));
        let mut victim_health = combat_query.get_mut(*victim).unwrap().0;

        if victim_health.current >= 1 {
            victim_health.current -= 1;
        }
        if victim_health.current < 1 && *victim != player {
            commands.entity(*victim).despawn_recursive();
        }
    });
}