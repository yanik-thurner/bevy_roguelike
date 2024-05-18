use crate::core_logic::map::{Map, map_index, TileType};
use crate::prelude::*;
use crate::prelude::map::{MAP_HEIGHT, MAP_WIDTH};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
}

pub fn system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, mut global_rng: GRng) {
    let mut map_builder = MapBuilder::new();
    let map = map_builder.build(&mut global_rng);

    commands.insert_resource(map);

    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            commands.spawn((
                WallOrFloor,
                Position::new(x as i32, y as i32),
                SpriteSheetBundle {
                    texture: asset_server.load("dungeonfont.png"),
                    transform: Transform::from_xyz((x * 32) as f32, (y * 32) as f32, 0.0),
                    visibility: Visibility::Hidden,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: if map_builder.map.tiles[map_index(x as i32, y as i32)] == TileType::Wall { SPRITE_SHEET_WALL } else { SPRITE_SHEET_FLOOR },
                    },
                    ..default()
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

    fn build_fov_test(&mut self) -> Map {
        self.fill(TileType::Floor);
        let x = MAP_WIDTH as i32 / 2;
        let y = MAP_HEIGHT as i32 / 2;
        self.map.spawn_player = GridPosition::new(x, y);
        self.map.tiles[map_index(x - 4, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x - 3, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x - 2, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x - 1, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x + 0, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x + 1, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x + 2, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x + 3, y + 3)] = TileType::Wall;
        self.map.tiles[map_index(x + 3, y + 2)] = TileType::Wall;
        self.map.tiles[map_index(x + 3, y + 1)] = TileType::Wall;
        self.map.tiles[map_index(x + 4, y + 1)] = TileType::Wall;

        self.map.tiles[map_index(x + 3, y - 1)] = TileType::Wall;
        self.map.tiles[map_index(x + 4, y - 1)] = TileType::Wall;
        self.map.tiles[map_index(x + 4, y - 2)] = TileType::Wall;
        self.map.tiles[map_index(x + 3, y - 2)] = TileType::Wall;
        self.map.tiles[map_index(x + 2, y - 2)] = TileType::Wall;

        self.map.tiles[map_index(x - 0, y - 2)] = TileType::Wall;
        self.map.tiles[map_index(x - 1, y - 2)] = TileType::Wall;
        self.map.tiles[map_index(x - 2, y - 2)] = TileType::Wall;
        self.map.tiles[map_index(x - 2, y - 3)] = TileType::Wall;
        self.map.tiles[map_index(x - 2, y - 4)] = TileType::Wall;

        self.map.tiles[map_index(x - 2, y - 0)] = TileType::Wall;
        self.map.tiles[map_index(x - 2, y + 1)] = TileType::Wall;
        self.map.tiles[map_index(x - 3, y + 1)] = TileType::Wall;
        self.map.tiles[map_index(x - 4, y + 1)] = TileType::Wall;
        self.map.clone()
    }
    fn build(&mut self, rng: &mut GRng) -> Map {
        self.fill(TileType::Wall);
        self.build_random_rooms(rng);
        self.build_corridors(rng);
        self.map.spawn_player = self.rooms[0].center().into();
        self.map.rooms = self.rooms.clone();

        let dijkstra_map = path_finding::create_dijkstra_map(&self.map, &self.map.spawn_player);

        if let Some((x, y, _)) = dijkstra_map.iter().enumerate()
            .flat_map(
                |(col, row_values)| {
                    row_values.iter().enumerate().filter_map(move |(row, &val)| {
                        val.map(|v| (row, col, v))
                    })
                })
            .max_by_key(|&(_, _, val)| val) {
            self.map.spawn_amulet = GridPosition::new(x as i32, y as i32);
        }

        self.map.clone()
    }
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut GRng) {
        while self.rooms.len() < NUM_ROOMS {
            let x = RngHelper::generate_from_range(rng.next_u32(), 1..MAP_WIDTH as i32 - 10);
            let y = RngHelper::generate_from_range(rng.next_u32(), 1..MAP_HEIGHT as i32 - 10);
            let width = RngHelper::generate_from_range(rng.next_u32(), 1..=10);
            let height = RngHelper::generate_from_range(rng.next_u32(), 1..=10);
            let room = Rect::new(x as f32, y as f32, (x + width) as f32, (y + height) as f32);

            let overlap = self.rooms.iter().any(|r| !r.intersect(room).is_empty());

            if !overlap {
                for i in x..x + width {
                    for j in y..y + height {
                        if i > 0 && i < MAP_WIDTH as i32
                            && j > 0 && j < MAP_HEIGHT as i32 {
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
            if let Some(idx) = self.map.try_idx(&GridPosition::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(&GridPosition::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut GRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.total_cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if RngHelper::generate_from_range(rng.next_u32(), 0..2) == 1 {
                self.apply_horizontal_tunnel(prev.x as i32, new.x as i32, prev.y as i32);
                self.apply_vertical_tunnel(prev.y as i32, new.y as i32, new.x as i32);
            } else {
                self.apply_vertical_tunnel(prev.y as i32, new.y as i32, prev.x as i32);
                self.apply_horizontal_tunnel(prev.x as i32, new.x as i32, new.y as i32);
            }
        }
    }
}