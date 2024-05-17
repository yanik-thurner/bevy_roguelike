use crate::prelude::*;

pub fn end_turn_system(current_turn_state: Res<State<TurnState>>, player_health: Query<&Health, With<Player>>, mut next_turn_state: ResMut<NextState<TurnState>>) {
    let current_state = current_turn_state.get().clone();
    let mut following_state = match current_state {
        TurnState::Init => TurnState::AwaitingInput,
        TurnState::AwaitingInput => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    if let Some(player_health) = player_health.get_single().ok() {
        if player_health.current < 1 && current_state != TurnState::Init {
            following_state = TurnState::GameOver;
        }
    }

    println!("Switching TurnState: {:?} -> {:?}", current_state, following_state);
    next_turn_state.set(following_state);
}