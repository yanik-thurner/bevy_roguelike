use crate::prelude::*;

pub fn update_animations(mut commands: Commands, mut animations: Query<(Entity, &mut Animation, &mut Transform, &Position)>, time: Res<Time>) {
    for (entity, mut animation, mut current_transform, position) in animations.iter_mut() {
        let original_transform = Transform::from_xyz(position.x as f32 * SPRITE_SIZE, position.y as f32 * SPRITE_SIZE, current_transform.translation.z);

        animation.update(current_transform.as_mut(), &original_transform, time.delta());
        animation.remove_if_finished(&mut commands.entity(entity));
    }
}