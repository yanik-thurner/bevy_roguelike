use std::ops::Div;

use bevy::prelude::Resource;

use crate::prelude::Component;
use crate::prelude::encoding::YFirstEncoding;

use super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Resource, Clone, Eq, PartialEq)]
pub struct Map {
    pub tiles: Grid<TileType, YFirstEncoding>,
    pub revealed: Grid<bool, YFirstEncoding>,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Revealed;

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let min_xy = Position::new(-(width.div_ceil(2) as i32), -(height.div_ceil(2) as i32));
        let max_xy = Position::new(width.div(2) as i32 - 1, height.div(2) as i32 - 1);
        let tiles = Grid::new_with_minmax(min_xy, max_xy, TileType::Floor);

        Map {
            tiles,
            revealed: Grid::new_with_minmax(min_xy, max_xy, false),
        }
    }

    pub fn can_enter_tile(&self, position: &Position) -> bool {
        match self.tiles.try_get(position) {
            Some(tile) => *tile == TileType::Floor,
            None => false
        }
    }

    pub fn reveal(&mut self, position: &Position) -> Result<bool, String> {
        if self.revealed.contains_position(position) {
            self.revealed[position] = true;
            return Ok(true);
        }
        return Err(format!("Position ({:?} is out of map bounds!", position));
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.tiles.get_dimensions()
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.tiles.get_min(), self.tiles.get_max())
    }
}


