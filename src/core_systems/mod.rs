use crate::prelude::*;
use crate::system_sets::GameplaySet;

mod player_input;
mod sync_grid;
mod sync_cam;
mod collision;
mod random_move;
mod end_turn;

pub struct CoreSystems;

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.insert_state(TurnState::AwaitingInput);

        app.add_systems(Update, player_input::system.in_set(GameplaySet::AwaitingInput));

        app.add_systems(Update, (collision::system, end_turn::system).chain().in_set(GameplaySet::PlayerTurn));

        app.add_systems(Update, (random_move::system, collision::system, end_turn::system).chain().in_set(GameplaySet::MonsterTurn));

        app.add_systems(Update, sync_grid::system);
        app.add_systems(Update, sync_cam::system);

        app.configure_sets(Update, (
            GameplaySet::AwaitingInput.run_if(in_state(TurnState::AwaitingInput)),
            GameplaySet::PlayerTurn.run_if(in_state(TurnState::PlayerTurn)),
            GameplaySet::MonsterTurn.run_if(in_state(TurnState::MonsterTurn)),
        ));
    }
}