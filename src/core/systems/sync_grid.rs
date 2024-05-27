use crate::prelude::*;

pub fn sync_grid_system(mut query: Query<(&mut Transform, &Position), (Without<PlayerCamera>, Without<Animation>)>) {
    for (mut transform, grid_pos) in query.iter_mut() {
        transform.translation.x = grid_pos.x as f32 * SPRITE_SIZE;
        transform.translation.y = grid_pos.y as f32 * SPRITE_SIZE;
        transform.scale = Vec3::new(1.0, 1.0, 1.0);
    }
}