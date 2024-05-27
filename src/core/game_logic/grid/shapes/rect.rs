use std::cmp::{max, min};
use std::ops::RangeInclusive;

use super::super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
    min: Position,
    max: Position,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(min: Position, max: Position) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        Rect {
            min,
            max,
        }
    }
    pub fn with_moved_edges(&self, top_edge_delta: i32, right_edge_delta: i32, bottom_edge_delta: i32, left_edge_delta: i32) -> Self {
        let min_x = self.min.x + left_edge_delta;
        let max_x = self.max.x + right_edge_delta;
        let min_y = self.min.y + bottom_edge_delta;
        let max_y = self.max.y + top_edge_delta;

        Rect {
            min: Position::new(min(min_x, max_x), min(min_y, max_y)),
            max: Position::new(max(min_x, max_x), max(min_y, max_y)),
        }
    }

    pub fn with_margin(&self, margin: u32) -> Self {
        self.with_margin_asymmetric(margin, margin, margin, margin)
    }

    pub fn with_margin_asymmetric(&self, margin_top: u32, margin_right: u32, margin_bottom: u32, margin_left: u32) -> Self {
        self.with_moved_edges(margin_top as i32, margin_right as i32, -(margin_bottom as i32), -(margin_left as i32))
    }

    pub fn with_padding(&self, padding: u32) -> Result<Self, String> {
        self.with_padding_asymmetric(padding, padding, padding, padding)
    }

    pub fn with_padding_asymmetric(&self, padding_top: u32, padding_right: u32, padding_bottom: u32, padding_left: u32) -> Result<Self, String> {
        if padding_top + padding_bottom > self.height() || padding_left + padding_right > self.width() {
            return Err(format!("Padding (t:{}, r:{}, b:{}, l:{}) too large for rectangle ({:?})", padding_top, padding_right, padding_bottom, padding_left, self));
        }

        Ok(self.with_moved_edges(-(padding_top as i32), -(padding_right as i32), padding_bottom as i32, padding_left as i32))
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn width(&self) -> u32 {
        (self.max.x - self.min.x) as u32 + 1
    }

    pub fn height(&self) -> u32 {
        (self.max.y - self.min.y) as u32 + 1
    }

    pub fn min(&self) -> Position {
        self.min
    }

    pub fn max(&self) -> Position {
        self.max
    }

    pub fn contains_rect(&self, other: Rect) -> bool {
        self.contains_position(&other.min) && self.contains_position(&other.max)
    }
    pub fn contains_position(&self, position: &Position) -> bool {
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
        self.min.x <= other.max.x &&
            self.max.x >= other.min.x &&
            self.min.y <= other.max.y &&
            self.max.y >= other.min.y
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if self.intersects(other) {
            let min_x = max(self.min.x, other.min.x);
            let min_y = max(self.min.y, other.min.y);
            let max_x = min(self.max.x, other.max.x);
            let max_y = min(self.max.y, other.max.y);

            Some(Rect {
                min: Position { x: min_x, y: min_y },
                max: Position { x: max_x, y: max_y },
            })
        } else {
            None
        }
    }

    pub fn center(&self) -> Position {
        Position::new(self.min.x + (self.width() / 2) as i32,
                      self.min.y + (self.height() / 2) as i32)
    }
}

impl Shape for Rect {
    fn get_bounding_rect(&self) -> Rect {
        Rect {
            min: self.min,
            max: self.max,
        }
    }

    fn contains(&self, position: &Position) -> bool {
        self.contains_position(position)
    }

    fn center(&self) -> Position {
       self.center()
    }
}