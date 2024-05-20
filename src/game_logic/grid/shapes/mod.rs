use super::super::prelude::*;

mod circle;
mod rect;

pub mod prelude {
    pub use super::circle::*;
    pub use super::rect::*;
    pub use super::*;
}

pub trait Shape {
    fn get_bounding_rect(&self) -> Rect;

    fn contains(&self, position: &GridPosition) -> bool;

    //fn iter_points(&self) -> Iter<GridPosition>;

    //fn get_anchor(&self) -> Option<GridPosition>;
}