use crate::prelude::*;

pub fn system(map: Res<Map>, mut query: Query<&mut GridPosition, With<MovingRandomly>>) {
    for mut position in query.iter_mut() {
        let direction = GridPosition::random_direction(false);
        if map.can_enter_tile(*position + direction) {
            *position = *position + direction
        }
    }
}