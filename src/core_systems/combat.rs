use bevy::tasks::futures_lite::StreamExt;
use crate::prelude::*;

pub fn combat_system(mut commands: Commands, mut ev_attack: EventReader<WantsToAttack>, mut query: Query<(&mut Health, Entity)>, player_query: Query<Entity, With<Player>>, state: Res<State<TurnState>>) {
    let player = player_query.get_single().unwrap();
    let mut victims: Vec<_> = ev_attack.read()
        .filter_map(|x| {
            if x.attacker == player && *state.get() == TurnState::MonsterTurn {
                None
            } else {
                query.get(x.victim).ok()
            }
        })
        .map(|(health, entity)| entity)
        .collect();


    victims.iter().for_each(|victim| {
        let mut victim_health = query.get_mut(*victim).unwrap().0;

        victim_health.current -= 1;
        if victim_health.current < 1 {
            commands.entity(*victim).clear_children();
            commands.entity(*victim).despawn();
        }
    });
}