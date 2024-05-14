use bevy::render::view::RenderLayers;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::core_logic::map::{Map, map_index, TileType};

use crate::prelude::*;
use crate::prelude::map::{MAP_HEIGHT, MAP_WIDTH};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
}

pub fn system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let mut rng = ThreadRng::default();
    let mut map_builder = MapBuilder::new();
    let map = map_builder.build(&mut rng);


    commands.insert_resource(map);


    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            commands.spawn((
                GridPosition { x: x as i32, y: y as i32 },
                RenderBundle {
                    sprite: SpriteSheetBundle {
                        texture: asset_server.load("dungeonfont.png"),
                        transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, 0.0),
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: if map_builder.map.tiles[map_index(x as i32, y as i32)] == TileType::Wall { SPRITE_SHEET_WALL } else { SPRITE_SHEET_FLOOR },
                        },
                        ..default()
                    },
                    layer: RenderLayers::layer(0),
                },
            ));
        }
    }
}

impl MapBuilder {
    pub fn new() -> Self {
        let mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
        };
        mb
    }

    fn build(&mut self, rng: &mut ThreadRng) -> Map {
        self.fill(TileType::Wall);
        self.build_random_rooms(rng);
        self.build_corridors(rng);
        self.map.spawn = self.rooms[0].center().into();
        self.map.rooms = self.rooms.clone();
        self.map.clone()
    }
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut ThreadRng) {
        while self.rooms.len() < NUM_ROOMS {
            let x = rng.gen_range(1..MAP_WIDTH - 10);
            let y = rng.gen_range(1..MAP_HEIGHT - 10);
            let width = rng.gen_range(1..=10);
            let height = rng.gen_range(1..=10);
            let room = Rect::new(x as f32, y as f32, (x + width) as f32, (y + height) as f32);

            let overlap = self.rooms.iter().any(|r| !r.intersect(room).is_empty());

            if !overlap {
                for i in x..x + width {
                    for j in y..y + height {
                        if i > 0 && i < MAP_WIDTH
                            && j > 0 && j < MAP_HEIGHT {
                            self.map.tiles[map_index(i as i32, j as i32)] = TileType::Floor;
                        }
                    }
                }
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(GridPosition::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(GridPosition::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut ThreadRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.total_cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.gen_bool(0.5) {
                self.apply_horizontal_tunnel(prev.x as i32, new.x as i32, prev.y as i32);
                self.apply_vertical_tunnel(prev.y as i32, new.y as i32, new.x as i32);
            } else {
                self.apply_vertical_tunnel(prev.y as i32, new.y as i32, prev.x as i32);
                self.apply_horizontal_tunnel(prev.x as i32, new.x as i32, new.y as i32);
            }
        }
    }
}