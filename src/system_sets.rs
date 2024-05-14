use crate::components::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySet {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}