use std::ops::Add;

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

pub fn player_input_system(keyboard_input: Res<ButtonInput<KeyCode>>,
                           time: Res<Time>,
                           mut player_query: Query<(Entity, &mut Player, &GridPosition, &mut Health)>,
                           enemy_query: Query<(&GridPosition, Entity), With<Enemy>>,
                           mut turn_state: ResMut<NextState<TurnState>>,
                           mut ev_wants_to_move: EventWriter<WantsToMoveEvent>,
                           mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>,
) {
    let (entity, mut player, position, mut health) = player_query.get_single_mut().unwrap();
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

        match delta {
            Some(GridPosition::ZERO) => {
                player.move_cooldown.reset();

                if health.current == health.before {
                    health.current = i32::min(health.max, health.current + 1);
                }
                health.before = health.current;
                turn_state.set(TurnState::PlayerTurn);
            }
            Some(delta) => {
                player.move_cooldown.reset();
                let new_position = *position + delta;

                let target = enemy_query
                    .iter()
                    .filter(|(pos, _)| **pos == new_position)
                    .map(|(_, enemy)| enemy)
                    .next();


                if let Some(enemy) = target {
                    ev_wants_to_attack.send(WantsToAttackEvent { attacker: entity, victim: enemy });
                    turn_state.set(TurnState::PlayerTurn);
                } else {
                    ev_wants_to_move.send(WantsToMoveEvent { entity, destination: new_position });
                    turn_state.set(TurnState::PlayerTurn);
                }
            }
            None => {}
        }
    }
}