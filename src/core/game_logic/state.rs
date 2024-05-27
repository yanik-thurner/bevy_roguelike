use bevy::prelude::States;

#[derive(States, Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    WorldTurn,
}

impl TurnState {
    pub fn next(&self) -> Self {
        match self {
            TurnState::AwaitingInput => TurnState::AwaitingInput,
            TurnState::PlayerTurn => TurnState::WorldTurn,
            TurnState::WorldTurn => TurnState::AwaitingInput
        }
    }
}

#[derive(States, Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum PlayState {
    Playing,
    Victory,
    Defeat,
}
