use std::ops::{Add, AddAssign};
use super::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(GridPosition);

impl Position {
    pub fn from_grid_position(grid_position: GridPosition) -> Self {
        Position(grid_position)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Position(GridPosition::new(x, y))
    }
    pub fn get(&self) -> &GridPosition {
        &self.0
    }
}

impl Add<&Self> for Position {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Position(self.0 + &rhs.0)
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<&Self> for Position {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += &rhs.0
    }
}

impl AddAssign<Self> for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs
    }
}

impl Add<&GridDirection> for Position {
    type Output = Position;

    fn add(self, rhs: &GridDirection) -> Self::Output {
        Position(self.0 + rhs)
    }
}

impl Add<GridDirection> for Position {
    type Output = Position;

    fn add(self, rhs: GridDirection) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<&GridDirection> for Position {
    fn add_assign(&mut self, rhs: &GridDirection) {
        self.0 += rhs
    }
}

impl AddAssign<GridDirection> for Position {
    fn add_assign(&mut self, rhs: GridDirection) {
        *self += &rhs
    }
}