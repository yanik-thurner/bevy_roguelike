use crate::prelude::*;

const SPEED: f32 = 25.0;

pub fn sync_grid_system(mut query: Query<(&mut Transform, &GridPosition), Without<PlayerCamera>>, time: Res<Time>) {
    for (mut transform, grid_pos) in query.iter_mut() {
        let target_position = Vec3::new(grid_pos.x as f32 * 32.0, grid_pos.y as f32 * 32.0, transform.translation.z);
        transform.translation.x += (target_position.x - transform.translation.x) * (1.0 - (-SPEED * time.delta_seconds()).exp());
        transform.translation.y += (target_position.y - transform.translation.y) * (1.0 - (-SPEED * time.delta_seconds()).exp());
    }
}