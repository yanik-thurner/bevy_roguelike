use crate::core::game_logic::grid::direction::Direction;
use crate::prelude::*;

const INPUT_KEYS: [KeyCode; 9] = [
    KeyCode::Numpad8,
    KeyCode::Numpad9,
    KeyCode::Numpad6,
    KeyCode::Numpad3,
    KeyCode::Numpad2,
    KeyCode::Numpad1,
    KeyCode::Numpad4,
    KeyCode::Numpad7,
    KeyCode::Numpad5,
];

fn map_key_code_to_direction(key_code: &KeyCode) -> Option<Direction> {
    match key_code {
        KeyCode::Numpad8 => Some(Direction::NORTH),
        KeyCode::Numpad9 => Some(Direction::NORTH_EAST),
        KeyCode::Numpad6 => Some(Direction::EAST),
        KeyCode::Numpad3 => Some(Direction::SOUTH_EAST),
        KeyCode::Numpad2 => Some(Direction::SOUTH),
        KeyCode::Numpad1 => Some(Direction::SOUTH_WEST),
        KeyCode::Numpad4 => Some(Direction::WEST),
        KeyCode::Numpad7 => Some(Direction::NORTH_WEST),
        KeyCode::Numpad5 => Some(Direction::NONE),
        _ => None
    }
}

pub fn process_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>,
                              mut turn_state: ResMut<NextState<TurnState>>,
                              time: Res<Time>,
                              mut player_query: Query<(Entity, &Position, &mut InputCooldown), With<Player>>,
                              enemy_query: Query<(&Position, Entity), With<Enemy>>,
                              mut ev_wants_to_move: EventWriter<WantsToMoveEvent>,
                              mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>,
                              mut commands: Commands) {
    if player_query.is_empty() {
        return;
    }
    let (entity, position, mut input_cooldown) = player_query.get_single_mut().unwrap();

    if keyboard_input.any_just_pressed(INPUT_KEYS) {
        input_cooldown.timer.reset();
    } else if !input_cooldown.timer.finished() {
        input_cooldown.timer.tick(time.delta());
        return;
    }

    let mut delta: Option<Direction> = None;
    for key_press in keyboard_input.get_pressed() {
        if delta.is_some() {
            break;
        }
        delta = map_key_code_to_direction(key_press);
    }

    if let Some(delta) = delta {
        if delta != Direction::NONE {
            let new_position = *position + &delta;

            let target = enemy_query
                .iter()
                .filter(|(pos, _)| **pos == new_position)
                .map(|(_, enemy)| enemy)
                .next();

            if let Some(enemy) = target {
                ev_wants_to_attack.send(WantsToAttackEvent { attacker: entity, victim: enemy, direction: delta });
            } else {
                ev_wants_to_move.send(WantsToMoveEvent { entity, destination: new_position, direction: delta });
            }
        } else {
            commands.entity(entity).insert(Animation::new_wait_animation());
        }
        println!("{:?} => {:?}", TurnState::AwaitingInput, TurnState::PlayerTurn);
        turn_state.set(TurnState::PlayerTurn)
    }
}
