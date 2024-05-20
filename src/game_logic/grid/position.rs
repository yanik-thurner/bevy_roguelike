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