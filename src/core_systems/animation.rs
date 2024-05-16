use crate::prelude::*;

pub fn combat_animation_system(mut commands: Commands, mut query: Query<&mut Attacker>, attacks: Query<Entity, With<Attack>>, time: Res<Time>) {
    for mut attacker in query.iter_mut() {
        attacker.cooldown.tick(time.delta());

        if let Some(attack) = attacker.attack {
            if attacker.cooldown.finished() && attacks.contains(attack) {
                commands.entity(attack).despawn();
            }
        }
    }
}