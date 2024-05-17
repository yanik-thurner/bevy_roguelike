use crate::prelude::*;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Resource, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub spawn_player: GridPosition,
    pub spawn_amulet: GridPosition,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
            spawn_player: GridPosition::new(0, 0),
            spawn_amulet: GridPosition::new(0, 0),
            rooms: Vec::new(),
        }
    }

    pub fn in_bounds(&self, point: GridPosition) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH.try_into().unwrap()
            && point.y >= 0 && point.y < MAP_HEIGHT.try_into().unwrap()
    }

    pub fn can_enter_tile(&self, point: GridPosition) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x as i32, point.y as i32)] == TileType::Floor
    }

    pub fn try_idx(&self, point: GridPosition) -> Option<usize> {
        if self.in_bounds(point) {
            Option::Some(map_index(point.x as i32, point.y as i32))
        } else {
            Option::None
        }
    }
}

pub fn map_index(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH as i32 + x) as usize
}