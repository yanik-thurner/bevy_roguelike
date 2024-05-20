use crate::prelude::*;

pub fn player_input_system(keyboard_input: Res<ButtonInput<KeyCode>>,
                           time: Res<Time>,
                           mut player_query: Query<(Entity, &mut Player, &Position, &mut Health)>,
                           enemy_query: Query<(&Position, Entity), With<Enemy>>,
                           mut turn_state: ResMut<NextState<TurnState>>,
                           mut ev_wants_to_move: EventWriter<WantsToMoveEvent>,
                           mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>,
) {
    if player_query.is_empty() {
        return;
    }
    let (entity, mut player, position, mut health) = player_query.get_single_mut().unwrap();
    if player.move_cooldown.tick(time.delta()).finished() {
        let delta = if keyboard_input.pressed(KeyCode::ArrowUp) {
            Some(GridDirection::NORTH)
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Some(GridDirection::SOUTH)
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Some(GridDirection::WEST)
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Some(GridDirection::EAST)
        } else if keyboard_input.pressed(KeyCode::Space) {
            Some(GridDirection::NONE)
        } else {
            None
        };

        match delta {
            Some(GridDirection::NONE) => {
                player.move_cooldown.reset();

                turn_state.set(TurnState::PlayerTurn);
            }
            Some(delta) => {
                player.move_cooldown.reset();

                let new_position = position.0 + &delta;

                let target = enemy_query
                    .iter()
                    .filter(|(pos, _)| pos.0 == new_position)
                    .map(|(_, enemy)| enemy)
                    .next();


                if let Some(enemy) = target {
                    ev_wants_to_attack.send(WantsToAttackEvent { attacker: entity, victim: enemy });
                    turn_state.set(TurnState::PlayerTurn);
                } else {
                    ev_wants_to_move.send(WantsToMoveEvent { entity, destination: Position(new_position) });
                    turn_state.set(TurnState::PlayerTurn);
                }
            }
            None => {}
        }
    }
}