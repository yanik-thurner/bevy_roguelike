use crate::prelude::*;

pub fn end_turn_system(current_turn_state: Res<State<TurnState>>, mut next_turn_state: ResMut<NextState<TurnState>>) {
    let current_state = current_turn_state.get().clone();
    let following_state = match current_state {
        TurnState::AwaitingInput => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    next_turn_state.set(following_state);
}