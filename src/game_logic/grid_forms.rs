use std::cmp::min;
use std::ops::RangeInclusive;

use super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
    min: GridPosition,
    max: GridPosition,
}

impl Rect {
    pub fn new(min: GridPosition, max: GridPosition) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        Rect {
            min,
            max,
        }
    }
    pub fn with_padding(&self, padding: u32) -> Self {
        let max_x_padding = (self.width() / 2) - 1;
        let max_y_padding = (self.height() / 2) - 1;
        let x_padding = min(padding, max_x_padding) as i32;
        let y_padding = min(padding, max_y_padding) as i32;
        Rect {
            min: GridPosition::new(self.min.x + x_padding,
                                   self.min.y + y_padding),
            max: GridPosition::new(self.max.x - x_padding,
                                   self.max.y - y_padding),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn width(&self) -> u32 {
        (self.max.x - self.min.x) as u32
    }

    pub fn height(&self) -> u32 {
        (self.max.y - self.min.y) as u32
    }

    pub fn min(&self) -> GridPosition {
        self.min
    }

    pub fn max(&self) -> GridPosition {
        self.max
    }

    pub fn contains(&self, position: &GridPosition) -> bool {
        self.contains_xy(position.x, position.y)
    }

    pub fn contains_xy(&self, x: i32, y: i32) -> bool {
        x >= self.min.x && x <= self.max.x
            && y >= self.min.y && y <= self.max.y
    }

    pub fn x_range(&self) -> RangeInclusive<i32>
    {
        self.min.x..=self.max.x
    }

    pub fn y_range(&self) -> RangeInclusive<i32>
    {
        self.min.y..=self.max.y
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.min.x < other.max.x &&
            self.max.x > other.min.x &&
            self.min.y < other.max.y &&
            self.max.y > other.min.y
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if self.intersects(other) {
            let min_x = self.min.x.max(other.min.x);
            let min_y = self.min.y.max(other.min.y);
            let max_x = self.max.x.min(other.max.x);
            let max_y = self.max.y.min(other.max.y);

            Some(Rect {
                min: GridPosition { x: min_x, y: min_y },
                max: GridPosition { x: max_x, y: max_y },
            })
        } else {
            None
        }
    }

    pub fn center(&self) -> GridPosition {
        GridPosition::new(self.min.x + (self.width() / 2) as i32,
                          self.min.y + (self.height() / 2) as i32)
    }
}