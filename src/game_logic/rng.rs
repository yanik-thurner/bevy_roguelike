use std::collections::Bound;
use std::ops::RangeBounds;
use num_traits::{Bounded, PrimInt, cast::FromPrimitive};

use rand_core::{RngCore, SeedableRng};
use wyrand::WyRand;

use super::prelude::*;

pub struct RandomUtil;


impl RandomUtil {
    fn derive_four_u32(seed: u32) -> (u32, u32, u32, u32) {
        // Perform bitwise operations to derive new seeds
        let derived1 = seed.wrapping_add(seed.rotate_left(5) ^ 0xA5A5A5A5);
        let derived2 = derived1.wrapping_mul(0x6D2B79F5);
        let derived3 = derived2.wrapping_add(seed.rotate_right(7) ^ 0x3C3C3C3C);
        let derived4 = derived3.wrapping_mul(0xC6EF3720);

        (derived1, derived2, derived3, derived4)
    }

    fn pick_from_range<T: Bounded + PrimInt + FromPrimitive>(random: u32, range: impl RangeBounds<T>) -> T {
        let min = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start.add(T::one()),
            Bound::Unbounded => T::min_value(),
        };
        let max = match range.end_bound() {
            Bound::Included(&end) => end,
            Bound::Excluded(&end) => end.sub(T::one()),
            Bound::Unbounded => T::max_value(),
        };
        assert!(min != T::min_value() || max != T::max_value(), "Both ends unbounded leads to overflow");
        assert!(min <= max, "min should not be greater than max");

        let length = max.sub(min).saturating_add(T::one()).to_u32().unwrap();

        min.saturating_add(FromPrimitive::from_u32(random % length).unwrap())
    }

    pub fn binary(random: u32) -> bool {
        random % 2 == 0
    }

    pub fn true_or_false(random: u32, chance_for_true: f32) -> bool {
        assert!(chance_for_true >= 0.0 && chance_for_true <= 1.0);
        let random_value = (random as f64 / u32::MAX as f64) as f32;

        random_value < chance_for_true
    }

    pub fn rect_by_xy_and_dimensions<B: RangeBounds<i32>, C: RangeBounds<u32>>(random: u32, x_range: B, y_range: B, width_range: C, height_range: C) -> Rect {
        let (seed1, seed2, seed3, seed4) = Self::derive_four_u32(random);
        let x = Self::pick_from_range(seed1, x_range);
        let y = Self::pick_from_range(seed2, y_range);
        let width = Self::pick_from_range(seed3, width_range);
        let height = Self::pick_from_range(seed4, height_range);
        Rect::new(GridPosition::new(x, y), GridPosition::new(x + width as i32, y + height as i32))
    }

    pub fn rect_inside_rect<C: RangeBounds<u32>>(random: u32, mut surrounding_rect: Rect, width_range: C, height_range: C) -> Result<Rect, String> {
        let max_width = match width_range.end_bound() {
            Bound::Included(&end) => end,
            Bound::Excluded(&end) => end - 1,
            Bound::Unbounded => u32::MAX
        };
        let max_height = match height_range.end_bound() {
            Bound::Included(&end) => end,
            Bound::Excluded(&end) => end - 1,
            Bound::Unbounded => u32::MAX
        };

        let overflow_restriction = surrounding_rect.with_padding_asymmetric(max_height, max_width, 0, 0);
        if let Ok(rect) = overflow_restriction {
            Ok(Self::rect_by_xy_and_dimensions(random, rect.x_range(), rect.y_range(), width_range, height_range))
        } else {
            Err(format!("Width- (..={}) or Height-Ranges (..={}) could result in bigger rectangle than the surrounding one ({:?})!", max_width, max_height, &surrounding_rect))
        }
    }
}

pub struct Random
{
    rng: WyRand,
}

impl Random
{
    pub fn new(seed: u64) -> Self {
        Random {
            rng: WyRand::seed_from_u64(seed)
        }
    }

    pub fn pick_from_range<B: RangeBounds<i32>>(&mut self, range: B) -> i32 {
        RandomUtil::pick_from_range(self.rng.next_u32(), range)
    }

    pub fn rect_inside_rect<C: RangeBounds<u32>>(&mut self, surrounding_rect: Rect, width_range: C, height_range: C) -> Result<Rect, String> {
        RandomUtil::rect_inside_rect(self.rng.next_u32(), surrounding_rect, width_range, height_range)
    }

    pub fn binary(&mut self) -> bool {
        RandomUtil::binary(self.rng.next_u32())
    }
}