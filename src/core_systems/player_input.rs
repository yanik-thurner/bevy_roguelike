use std::ops::Add;
use crate::core_logic::map::Map;

use crate::prelude::*;

impl Add for GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: Self) -> Self::Output {
        GridPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn player_input_system(keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<(Entity, &mut Player, &mut GridPosition)>, mut turn_state: ResMut<NextState<TurnState>>, mut ev_wants_to_move: EventWriter<WantsToMoveEvent>) {
    let (entity, mut player, mut position) = query.get_single_mut().unwrap();
    if player.move_cooldown.tick(time.delta()).finished() {
        let delta = if keyboard_input.pressed(KeyCode::ArrowUp) {
            Some(GridPosition::NORTH)
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Some(GridPosition::SOUTH)
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Some(GridPosition::WEST)
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Some(GridPosition::EAST)
        } else if keyboard_input.pressed(KeyCode::Space) {
            Some(GridPosition::ZERO)
        } else {
            None
        };
        if let Some(delta) = delta
        {
            player.move_cooldown.reset();
            let new_position = *position + delta;
            ev_wants_to_move.send(WantsToMoveEvent { entity, destination: new_position });
            turn_state.set(TurnState::PlayerTurn);
        }
    }
}