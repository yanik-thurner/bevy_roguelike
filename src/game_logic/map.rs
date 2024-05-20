use std::collections::{HashMap, HashSet};
use std::ops::Div;
use std::slice::{Iter, IterMut};

use super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    position: GridPosition,
}

impl Tile {
    pub fn new(tile_type: TileType, position: GridPosition) -> Self {
        Self {
            tile_type,
            position,
        }
    }

    pub fn get_pos(&self) -> &GridPosition {
        &self.position
    }

    pub fn get_x(&self) -> i32 {
        self.position.x
    }

    pub fn get_y(&self) -> i32 {
        self.position.y
    }
}

pub struct TileBuilder;

impl TileBuilder {
    pub fn build_floor(position: GridPosition) -> Tile {
        Tile::new(TileType::Floor, position)
    }
    pub fn build_wall(position: GridPosition) -> Tile {
        Tile::new(TileType::Wall, position)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    dimension: (usize, usize),
    min_xy: GridPosition,
    max_xy: GridPosition,
    tiles: Vec<Tile>,
    revealed: Vec<bool>,
    xy_to_idx: HashMap<(i32, i32), usize>,
    idx_to_xy: HashMap<usize, (i32, i32)>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let min_xy = GridPosition::new(-(width.div_ceil(2) as i32), -(height.div_ceil(2) as i32));
        let max_xy = GridPosition::new(width.div(2) as i32 - 1, height.div(2) as i32 - 1);
        let mut xy_to_idx = HashMap::new();
        let mut idx_to_xy = HashMap::new();
        let mut tiles = vec![TileBuilder::build_wall(GridPosition::new(0, 0)); width * height];


        for idx in 0..(width * height) {
            let encoding = encoding::y_first;
            let (x, y) = encoding(idx, width, height);
            tiles[idx] = TileBuilder::build_floor(GridPosition::new(x, y));
            xy_to_idx.insert((x, y), idx);
            idx_to_xy.insert(idx, (x, y));
        }

        Map {
            dimension: (width, height),
            min_xy,
            max_xy,
            tiles,
            revealed: vec![false; width * height],
            xy_to_idx,
            idx_to_xy,
        }
    }

    pub fn is_position_in_bounds(&self, position: &GridPosition) -> bool {
        position.x >= self.min_xy.x && position.y >= self.min_xy.y
            && position.x <= self.max_xy.x && position.y <= self.max_xy.y
    }

    pub fn is_step_in_bounds(&self, position: &GridPosition, direction: &GridDirection) -> bool {
        self.is_position_in_bounds(&(*position + *direction))
    }

    pub fn can_enter_tile(&self, position: &GridPosition) -> bool {
        match self.try_get_tile(position) {
            Some(tile) => tile.tile_type == TileType::Floor,
            None => false
        }
    }

    pub fn try_get_tile_type(&self, position: &GridPosition) -> Option<TileType> {
        match self.is_position_in_bounds(position) {
            true => Some(self.tiles[self.pos_to_idx(position)].tile_type),
            false => None
        }
    }
    pub fn try_get_tile(&self, position: &GridPosition) -> Option<Tile> {
        match self.is_position_in_bounds(position) {
            true => Some(self.tiles[self.pos_to_idx(position)]),
            false => None
        }
    }

    pub fn set_tile<'a>(&mut self, position: &'a GridPosition, tile_type: TileType) -> Result<(&'a GridPosition, TileType), String> {
        if self.is_position_in_bounds(position) {
            let idx = self.pos_to_idx(position);
            self.tiles[idx].tile_type = tile_type;
            return Ok((position, tile_type));
        }
        return Err(format!("Position ({:?} is out of map bounds!", position));
    }

    pub fn reveal(&mut self, position: &GridPosition) -> Result<bool, String> {
        if self.is_position_in_bounds(position) {
            let idx = self.pos_to_idx(position);
            self.revealed[idx] = true;
            return Ok(true);
        }
        return Err(format!("Position ({:?} is out of map bounds!", position));
    }

    pub fn is_position_revealed(&self, position: &GridPosition) -> Result<bool, String> {
        if self.is_position_in_bounds(position) {
            let idx = self.pos_to_idx(position);
            return Ok(self.revealed[idx]);
        }

        return Err(format!("Position ({:?} is out of map bounds!", position));
    }

    pub fn pos_to_idx(&self, pos: &GridPosition) -> usize {
        self.xy_to_idx.get(&(pos.x, pos.y)).unwrap().clone()
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.dimension
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.min_xy, self.max_xy)
    }

    pub fn iter_tiles(&self) -> Iter<'_, Tile> {
        self.tiles.iter()
    }
    pub fn iter_mut_tiles(&mut self) -> IterMut<'_, Tile> {
        self.tiles.iter_mut()
    }
}


