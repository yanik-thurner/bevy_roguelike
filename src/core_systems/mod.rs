use crate::prelude::*;
use crate::system_sets::GameplaySet;

mod player_input;
mod sync_grid;
mod sync_cam;
mod random_move;
mod end_turn;
mod movement;
mod hud;
mod debug;
mod hud_enemies;
mod combat;
mod animation;

pub struct CoreSystems;

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.insert_state(TurnState::AwaitingInput);
        app.add_event::<WantsToMoveEvent>();
        app.add_event::<WantsToAttack>();

        app.add_systems(Startup, hud::setup_hud);

        app.add_systems(Update, player_input::player_input_system.in_set(GameplaySet::AwaitingInput));

        app.add_systems(Update, (movement::movement_system,
                                 combat::combat_system,
                                 end_turn::end_turn_system)
            .chain()
            .after(GameplaySet::AwaitingInput)
            .in_set(GameplaySet::PlayerTurn));

        app.add_systems(Update, (random_move::random_system,
                                 movement::movement_system,
                                 combat::combat_system,
                                 end_turn::end_turn_system)
            .chain()
            .after(GameplaySet::AwaitingInput)
            .after(GameplaySet::PlayerTurn)
            .in_set(GameplaySet::MonsterTurn));

        app.add_systems(Update, sync_grid::sync_grid_system.after(player_input::player_input_system));
        app.add_systems(Update, sync_cam::sync_cam_system.after(sync_grid::sync_grid_system));
        app.add_systems(Update, animation::combat_animation_system);
        app.add_systems(Update, hud::update_healthbar);
        app.add_systems(Update, hud_enemies::update_enemy_hud);


        app.configure_sets(Update, (
            GameplaySet::AwaitingInput.run_if(in_state(TurnState::AwaitingInput)),
            GameplaySet::PlayerTurn.run_if(in_state(TurnState::PlayerTurn)),
            GameplaySet::MonsterTurn.run_if(in_state(TurnState::MonsterTurn)),
        ));
    }
}