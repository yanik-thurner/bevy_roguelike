use crate::prelude::*;

const SPEED: f32 = 25.0;

pub fn sync_grid_system(mut commands: Commands,
                        mut query: Query<(Entity, &mut Transform, &mut Visibility, &Position, Option<&Enemy>), Without<PlayerCamera>>,
                        mut revealed: Query<&mut Sprite, (With<WallOrFloor>, With<Revealed>)>,
                        player: Query<&FieldOfView, With<Player>>,
                        time: Res<Time>) {
    for (entity, mut transform, mut visibility, grid_pos, enemy) in query.iter_mut() {
        let target_position = Vec3::new(grid_pos.0.x as f32 * SPRITE_SIZE, grid_pos.0.y as f32 * SPRITE_SIZE, transform.translation.z);
        transform.translation.x += (target_position.x - transform.translation.x) * (1.0 - (-SPEED * time.delta_seconds()).exp());
        transform.translation.y += (target_position.y - transform.translation.y) * (1.0 - (-SPEED * time.delta_seconds()).exp());


        if let Some(fov) = player.get_single().ok() {
            if fov.visible_tiles.contains(&grid_pos.0) {
                *visibility = Visibility::Visible;
                commands.entity(entity).insert(Revealed);
            } else if enemy.is_some() {
                *visibility = Visibility::Hidden;
            }

            if let Ok(mut sprite) = revealed.get_mut(entity) {
                match fov.visible_tiles.contains(&grid_pos.0) {
                    true => { sprite.color.set_a(1.0); }
                    false => { sprite.color.set_a(0.5); }
                }
            }
        }
    }
}