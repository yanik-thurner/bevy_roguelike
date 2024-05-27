use std::ops::{Add, AddAssign};

use super::position::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub const NONE: Direction = Direction::new(0, 0);
    pub const NORTH: Direction = Direction::new(0, 1);
    pub const NORTH_EAST: Direction = Direction::new(1, 1);
    pub const EAST: Direction = Direction::new(1, 0);
    pub const SOUTH_EAST: Direction = Direction::new(1, -1);
    pub const SOUTH: Direction = Direction::new(0, -1);
    pub const SOUTH_WEST: Direction = Direction::new(-1, -1);
    pub const WEST: Direction = Direction::new(-1, 0);
    pub const NORTH_WEST: Direction = Direction::new(-1, 1);

    pub const ALL_DIRECTIONS: [Direction; 8] = [
        Direction::NORTH,
        Direction::NORTH_EAST,
        Direction::EAST,
        Direction::SOUTH_EAST,
        Direction::SOUTH,
        Direction::SOUTH_WEST,
        Direction::WEST,
        Direction::NORTH_WEST];

    pub const ALL_DIRECTIONS_INCLUDING_NONE: [Direction; 9] = [
        Direction::NONE,
        Direction::NORTH,
        Direction::NORTH_EAST,
        Direction::EAST,
        Direction::SOUTH_EAST,
        Direction::SOUTH,
        Direction::SOUTH_WEST,
        Direction::WEST,
        Direction::NORTH_WEST];
    const fn new(x: i32, y: i32) -> Self {
        Direction {
            x,
            y,
        }
    }
}


impl Add<&Direction> for Position {
    type Output = Position;

    fn add(self, rhs: &Direction) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<&Direction> for Position {
    fn add_assign(&mut self, rhs: &Direction) {
        self.x += rhs.x;
        self.y = rhs.y;
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self += &rhs
    }
}