use std::ops::{Add, Deref};
use bevy::input::keyboard::KeyboardInput;
use crate::prelude::*;

impl Add for GridPosition {
    type Output = (GridPosition);

    fn add(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn system(keyboard_input: Res<ButtonInput<KeyCode>>, map: Res<Map>, time: Res<Time>, mut query: Query<(&mut Player, &mut GridPosition)>) {
    let (mut player, mut position) = query.get_single_mut().unwrap();
    if player.move_cooldown.tick(time.delta()).finished() {
        let delta = if keyboard_input.pressed(KeyCode::ArrowUp) {
            GridPosition::NORTH
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            GridPosition::SOUTH
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            GridPosition::WEST
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            GridPosition::EAST
        } else {
            GridPosition::ZERO
        };
        if delta != GridPosition::ZERO
        {
            player.move_cooldown.reset();
            let new_position = *position + delta;
            if map.can_enter_tile(new_position) {
                *position = *position + delta;
            }
        }
    }
}