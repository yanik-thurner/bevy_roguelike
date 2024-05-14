use crate::prelude::*;

pub fn end_turn_system(current_turn_state: Res<State<TurnState>>, mut next_turn_state: ResMut<NextState<TurnState>>) {
    let following_state = match current_turn_state.get() {
        TurnState::AwaitingInput => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput
    };

    next_turn_state.set(following_state);
}