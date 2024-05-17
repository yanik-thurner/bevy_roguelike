use crate::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TurnState {
    Init,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}