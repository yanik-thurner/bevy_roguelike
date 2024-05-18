use std::collections::Bound;
use std::ops::RangeBounds;

use crate::core::prelude::*;

pub type GRng<'a> = ResMut<'a, GlobalEntropy<WyRand>>;

#[derive(Component)]
pub struct RngSource;

pub fn setup_rng_source(mut commands: Commands, mut global_rng: ResMut<GlobalEntropy<WyRand>>) {
    commands.spawn((RngSource, global_rng.fork_rng()));
}

pub struct RngHelper;

impl RngHelper {
    pub fn generate_from_range<R: RangeBounds<i32>>(random: u32, range: R) -> i32
    {
        let min = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => i32::MIN,
        };
        let max = match range.end_bound() {
            Bound::Included(&end) => end,
            Bound::Excluded(&end) => end - 1,
            Bound::Unbounded => i32::MAX,
        };
        assert!(min <= max, "min should not be greater than max");

        let length = (max - min + 1) as u32;
        min + (random % length) as i32
    }
}