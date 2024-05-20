use std::ops::{Add, AddAssign};
use super::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(pub GridPosition);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(GridPosition::new(x, y))
    }

    pub fn grid_to_transform(&self) -> Transform{
        Transform::from_xyz(self.0.x as f32 * SPRITE_SIZE, self.0.y as f32 * SPRITE_SIZE, 0.0)
    }
}

