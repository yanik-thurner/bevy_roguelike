use crate::game_logic::rng::Random;

use super::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub spawn_player: GridPosition,
    pub spawn_amulet: GridPosition,
}

impl MapBuilder {
    const ROOM_TRIES: usize = 1_000;

    pub fn build(map_width: usize, map_height: usize, max_rooms: usize, seed: u64) -> Self {
        let mut rng = Random::new(seed);
        let mut mb = MapBuilder {
            map: Map::new(map_width, map_height),
            rooms: Vec::new(),
            spawn_player: GridPosition::new(0, 0),
            spawn_amulet: GridPosition::new(0, 0),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(max_rooms, &mut rng);
        mb.build_corridors(&mut rng);
        mb.spawn_player = mb.rooms[0].center();
        mb.spawn_amulet = mb.rooms.last().unwrap().center();

        mb
    }

    fn fill(&mut self, tile_type: TileType) {
        self.map.iter_mut_tiles().for_each(|t| t.tile_type = tile_type);
    }

    fn fill_rect(&mut self, rect: Rect, tile_type: TileType) {
        self.map.iter_mut_tiles()
            .filter(|t| rect.contains(t.get_pos()))
            .for_each(|t| t.tile_type = tile_type);
    }

    fn build_random_rooms(&mut self, max_rooms: usize, rng: &mut Random) {
        let mut tries: usize = 0;
        let map_rect_with_padding = self.map.get_rect().with_padding(1);
        while self.rooms.len() < max_rooms && tries < Self::ROOM_TRIES {
            let room = rng.rect_inside_rect(map_rect_with_padding, 3..10, 3..10);

            let overlap = self.rooms.iter().any(|r| r.intersects(&room));

            if !overlap {
                self.fill_rect(room, TileType::Floor);
                self.rooms.push(room);
            }

            tries += 1;
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            let _ = self.map.set_tile(&GridPosition::new(x, y), TileType::Floor);
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            let _ = self.map.set_tile(&GridPosition::new(x, y), TileType::Floor);
        }
    }

    fn build_corridors(&mut self, rng: &mut Random) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.binary() {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}