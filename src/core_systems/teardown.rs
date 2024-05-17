use crate::prelude::*;

pub fn teardown(mut commands: Commands, entities: Query<Entity, (Without<PlayerCamera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
