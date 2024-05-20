use bevy::ecs::schedule::SystemConfigs;

use crate::prelude::*;
use crate::system_sets::{GameplaySet, InitSet, ParallelSet};

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
mod chasing;
mod menu_screens;
mod fov;

pub struct CoreSystems;

fn init_system_bundle() -> SystemConfigs {
    (teardown, spawner::spawn_amulet_of_yala, spawner::spawn_random_monsters, hud::setup_hud, end_turn::end_turn_system)
        .chain()
        .in_set(InitSet)
        .before(GameplaySet::AwaitingInput)
        .before(GameplaySet::PlayerTurn)
        .before(GameplaySet::MonsterTurn)
        .before(ParallelSet)
}

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        // app.insert_state(TurnState::Init);
        // app.add_event::<WantsToMoveEvent>();
        // app.add_event::<WantsToAttackEvent>();
        //
        // app.add_systems(OnEnter(TurnState::Init), init_system_bundle().after(init_map));
        // app.add_systems(OnEnter(TurnState::GameOver), (teardown, menu_screens::game_over_screen).chain());
        // app.add_systems(OnEnter(TurnState::Victory), (teardown, menu_screens::victory_screen).chain());
        //
        // app.add_systems(Update, menu_screens::game_end_system.run_if(in_state(TurnState::GameOver).or_else(in_state(TurnState::Victory))));
        //
        // app.add_systems(Update, (player_input::player_input_system).after(ParallelSet).in_set(GameplaySet::AwaitingInput));
        //
        // app.add_systems(Update, (movement::movement_system,
        //                          combat::combat_system,
        //                          end_turn::end_turn_system)
        //     .chain()
        //     .after(ParallelSet)
        //     .after(GameplaySet::AwaitingInput)
        //     .in_set(GameplaySet::PlayerTurn));
        //
        // app.add_systems(Update, (random_move::random_system,
        //                          chasing::chase_player_system,
        //                          movement::movement_system,
        //                          combat::combat_system,
        //                          end_turn::end_turn_system)
        //     .chain()
        //     .after(ParallelSet)
        //     .after(GameplaySet::AwaitingInput)
        //     .after(GameplaySet::PlayerTurn)
        //     .in_set(GameplaySet::MonsterTurn));
        //
        // app.add_systems(Update, (
        //     sync_cam::sync_cam_system,
        //     animation::combat_animation_system,
        //     hud::update_healthbar,
        //     hud_enemies::update_enemy_hud.after(sync_cam::sync_cam_system)).in_set(ParallelSet));
        //
        // app.configure_sets(Update, (
        //     GameplaySet::AwaitingInput.run_if(in_state(TurnState::AwaitingInput)),
        //     GameplaySet::PlayerTurn.run_if(in_state(TurnState::PlayerTurn)),
        //     GameplaySet::MonsterTurn.run_if(in_state(TurnState::MonsterTurn)),
        //     ParallelSet.run_if(in_state(TurnState::AwaitingInput)
        //         .or_else(in_state(TurnState::PlayerTurn))
        //         .or_else(in_state(TurnState::MonsterTurn))),
        // ));
    }
}