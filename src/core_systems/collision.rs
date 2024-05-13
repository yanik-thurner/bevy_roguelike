use crate::prelude::*;

pub fn system(mut commands: Commands, player_position: Query<&GridPosition, With<Player>>, enemies: Query<(Entity, &GridPosition), (With<Enemy>, Without<Player>)>) {
    let player_position = player_position.get_single().unwrap();

    for (enemy, enemy_position) in enemies.iter() {
        if player_position == enemy_position {
            commands.entity(enemy).despawn();
        }
    }
}