use crate::core::prelude::*;

pub type GRng<'a> = ResMut<'a, GlobalEntropy<WyRand>>;

#[derive(Component)]
pub struct RngSource;

pub fn setup_rng_source(mut commands: Commands, mut global_rng: ResMut<GlobalEntropy<WyRand>>) {
    commands.spawn((RngSource, global_rng.fork_rng()));
}