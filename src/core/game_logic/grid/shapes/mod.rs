use super::super::prelude::*;

mod rect;

pub mod prelude {
    pub use super::*;
    pub use super::rect::*;
}
#[allow(dead_code)]
pub trait Shape: Send + Sync {
    fn get_bounding_rect(&self) -> Rect;

    fn contains(&self, position: &Position) -> bool;

    fn center(&self) -> Position;
}