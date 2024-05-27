use bevy::app::Plugin;
use bevy::ecs::schedule::SystemConfigs;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

use prelude::*;

mod systems;
mod assets;
mod structure;
mod layers;
mod cam;
pub mod game_logic;
mod display_logic;
mod events;
mod rng_source;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_prng::*;
    pub use bevy_rand::prelude::*;
    pub use rand_core::RngCore;

    pub use super::assets::*;
    pub use super::cam::*;
    pub use super::display_logic::prelude::*;
    pub use super::events::*;
    pub use super::game_logic::prelude::*;
    pub use super::layers::*;
    pub use super::rng_source::*;
    pub use super::structure::*;
    pub use super::systems::prelude::*;
}

pub struct Core;

const SEED: u64 = 0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitingInputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerTurnSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldTurnSet;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParallelSet;

trait SystemsCollection {
    fn systems() -> SystemConfigs;
}

impl SystemsCollection for AwaitingInputSet {
    fn systems() -> SystemConfigs {
        process_keyboard_input.in_set(AwaitingInputSet)
            .before(PlayerTurnSet)
            .before(WorldTurnSet)
    }
}

impl SystemsCollection for PlayerTurnSet {
    fn systems() -> SystemConfigs {
        (process_movement_events,
         fov_calculation,
         fov_revealing,
         combat_system,
         end_turn)
            .chain()
            .in_set(PlayerTurnSet)
            .before(WorldTurnSet)
    }
}

impl SystemsCollection for WorldTurnSet {
    fn systems() -> SystemConfigs {
        (fov_calculation,
         generate_action_events,
         process_movement_events,
         combat_system,
         end_turn)
            .chain()
            .in_set(WorldTurnSet)
    }
}

impl SystemsCollection for ParallelSet {
    fn systems() -> SystemConfigs {
        (sync_grid_system,
         sync_cam_system,
         update_health_bar,
         update_enemy_hud,
         update_animations,
        ).chain()
            .in_set(ParallelSet)
            .after(PlayerTurnSet)
            .after(WorldTurnSet)
    }
}

fn init() -> SystemConfigs {
    (teardown, init_map, init_enemies, init_cam, init_hud, fov_calculation, fov_revealing).chain()
}


impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(SEED.to_be_bytes()));
        app.add_systems(OnEnter(PlayState::Playing), (teardown, setup_rng_source, load_assets, init()).chain());
        app.add_systems(OnEnter(PlayState::Victory), (teardown, menu_screens::victory_screen).chain());
        app.add_systems(OnEnter(PlayState::Defeat), (teardown, menu_screens::game_over_screen).chain());
        app.add_systems(Update, menu_screens::game_end_system.run_if(in_state(PlayState::Defeat).or_else(in_state(PlayState::Victory))).before(PlayerTurnSet).before(WorldTurnSet));

        app.insert_state(TurnState::AwaitingInput);
        app.insert_state(PlayState::Playing);
        app.add_event::<WantsToMoveEvent>();
        app.add_event::<WantsToAttackEvent>();

        app.add_systems(Update, AwaitingInputSet::systems().run_if(in_state(PlayState::Playing)));
        app.add_systems(Update, PlayerTurnSet::systems().run_if(in_state(PlayState::Playing)));
        app.add_systems(Update, WorldTurnSet::systems().run_if(in_state(PlayState::Playing)));

        app.add_systems(Update, ParallelSet::systems().run_if(in_state(PlayState::Playing)));


        app.configure_sets(Update, (
            AwaitingInputSet.run_if(in_state(TurnState::AwaitingInput).and_then(in_state(PlayState::Playing))),
            PlayerTurnSet.run_if(in_state(TurnState::PlayerTurn).and_then(in_state(PlayState::Playing))),
            WorldTurnSet.run_if(in_state(TurnState::WorldTurn).and_then(in_state(PlayState::Playing))),
            ParallelSet.run_if(in_state(PlayState::Playing)),
        ));
    }
}
