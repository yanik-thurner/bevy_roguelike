use std::collections::HashMap;
use std::ops::{Add, AddAssign};

use bevy::prelude::Component;

use super::direction::Direction;

#[derive(Component, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y,
        }
    }

    pub fn from_tuple(t: (i32, i32)) -> Self {
        Position {
            x: t.0,
            y: t.1,
        }
    }
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn get_adjacent_positions(&self) -> HashMap<Direction, Position> {
        let mut map = HashMap::with_capacity(8);
        map.insert(Direction::NORTH, *self + Direction::NORTH);
        map.insert(Direction::NORTH_EAST, *self + Direction::NORTH_EAST);
        map.insert(Direction::EAST, *self + Direction::EAST);
        map.insert(Direction::SOUTH_EAST, *self + Direction::SOUTH_EAST);
        map.insert(Direction::SOUTH, *self + Direction::SOUTH);
        map.insert(Direction::SOUTH_WEST, *self + Direction::SOUTH_WEST);
        map.insert(Direction::WEST, *self + Direction::WEST);
        map.insert(Direction::NORTH_WEST, *self + Direction::NORTH_WEST);
        map
    }
}

impl Add<&Self> for Position {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Self> for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs
    }
}