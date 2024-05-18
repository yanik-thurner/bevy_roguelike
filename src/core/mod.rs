use bevy::app::{Plugin, Startup};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

use crate::components::App;

mod spatial;
mod rng;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_prng::*;
    pub use bevy_rand::prelude::*;
    pub use rand_core::RngCore;

    pub use crate::game_logic::prelude::*;

    pub use crate::core::spatial::*;

    pub use super::rng::*;
}

pub struct Core;

const SEED: u64 = 0;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(SEED.to_be_bytes()));
        app.add_systems(Startup, rng::setup_rng_source);
    }
}