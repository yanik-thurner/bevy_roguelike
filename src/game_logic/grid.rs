use std::ops::{Add, AddAssign};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridPosition
{
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition {
            x,
            y,
        }
    }

    pub fn from_tuple(t: (i32, i32)) -> Self {
        GridPosition {
            x: t.0,
            y: t.1,
        }
    }
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Add<&Self> for GridPosition {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        GridPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Self> for GridPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<&Self> for GridPosition {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Self> for GridPosition {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridDirection {
    x: i32,
    y: i32,
}

impl GridDirection {
    pub const NONE: GridDirection = GridDirection::new(0, 0);
    pub const NORTH: GridDirection = GridDirection::new(0, 1);
    pub const NORTH_EAST: GridDirection = GridDirection::new(1, 1);
    pub const EAST: GridDirection = GridDirection::new(1, 0);
    pub const SOUTH_EAST: GridDirection = GridDirection::new(1, -1);
    pub const SOUTH: GridDirection = GridDirection::new(0, -1);
    pub const SOUTH_WEST: GridDirection = GridDirection::new(-1, -1);
    pub const WEST: GridDirection = GridDirection::new(-1, 0);
    pub const NORTH_WEST: GridDirection = GridDirection::new(-1, 1);

    pub const ALL_DIRECTIONS: [GridDirection; 8] = [
        GridDirection::NORTH,
        GridDirection::NORTH_EAST,
        GridDirection::EAST,
        GridDirection::SOUTH_EAST,
        GridDirection::SOUTH,
        GridDirection::SOUTH_WEST,
        GridDirection::WEST,
        GridDirection::NORTH_WEST];

    const fn new(x: i32, y: i32) -> Self {
        GridDirection {
            x,
            y,
        }
    }
}


impl Add<&GridDirection> for GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: &GridDirection) -> Self::Output {
        GridPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<GridDirection> for GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: GridDirection) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<&GridDirection> for GridPosition {
    fn add_assign(&mut self, rhs: &GridDirection) {
        self.x += rhs.x;
        self.y = rhs.y;
    }
}

impl AddAssign<GridDirection> for GridPosition {
    fn add_assign(&mut self, rhs: GridDirection) {
        *self += &rhs
    }
}