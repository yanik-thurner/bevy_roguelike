use bevy::app::{Plugin, Startup};
use bevy::ecs::schedule::SystemConfigs;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

use crate::components::App;
use prelude::*;

mod spatial;
mod rng;
mod map;

mod systems;
mod assets;
mod structure;
mod layers;
mod actor;
mod cam;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_prng::*;
    pub use bevy_rand::prelude::*;
    pub use rand_core::RngCore;

    pub use crate::game_logic::prelude::*;

    pub use crate::core::spatial::*;
    pub use crate::core::map::*;

    pub use super::rng::*;
    pub use super::systems::prelude::*;
    pub use super::assets::*;
    pub use super::structure::*;

    pub use super::layers::*;
    pub use super::actor::*;
    pub use super::cam::*;
}

pub struct Core;

const SEED: u64 = 0;

fn init() -> SystemConfigs {
    (teardown, init_map, init_cam).chain()
}

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(SEED.to_be_bytes()));
        app.add_systems(Startup, (setup_rng_source, load_assets, init()).chain());
    }
}