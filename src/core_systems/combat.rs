use bevy::tasks::futures_lite::StreamExt;
use crate::prelude::*;


pub fn combat_system(mut commands: Commands, mut ev_attack: EventReader<WantsToAttack>, mut combat_query: Query<(&mut Health, Entity)>, mut animation_query: Query<(&Transform, &mut Attacker), (With<Health>, With<GridPosition>)>, player_query: Query<Entity, With<Player>>, state: Res<State<TurnState>>) {
    let player = player_query.get_single().unwrap();

    let mut attacker_victim_pairs: Vec<_> = ev_attack.read()
        .filter(|event| event.attacker == player && *state.get() == TurnState::MonsterTurn)
        .map(|event| (event.attacker, event.victim)).collect();

    for (attacker, victim) in attacker_victim_pairs.iter() {
        let (victim_transform, _) = animation_query.get(*victim).unwrap();
        let victim_transform = victim_transform.clone();
        let (attacker_transform, mut attacker_component) = animation_query.get_mut(*attacker).unwrap();
        attacker_component.cooldown.reset();
        attacker_component.attack = Some(commands.spawn((Attack, SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform::interpolate(attacker_transform, &victim_transform, 0.5),
            ..default()
        })).id());
    }


    attacker_victim_pairs.iter().for_each(|(attacker, victim)| {
        let mut victim_health = combat_query.get_mut(*victim).unwrap().0;

        victim_health.current -= 1;
        if victim_health.current < 1 {
            commands.entity(*victim).clear_children();
            commands.entity(*victim).despawn();
        }
    });
}