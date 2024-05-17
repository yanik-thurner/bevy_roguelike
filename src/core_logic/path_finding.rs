use crate::components::{GridPosition, Res};
use crate::components::map::{Map, MAP_HEIGHT, MAP_WIDTH};


const DIRECTIONS: [GridPosition; 4] = [GridPosition::NORTH, GridPosition::EAST, GridPosition::SOUTH, GridPosition::WEST];

#[allow(dead_code)]
fn print_dijkstra(dijkstra_map: &[[Option<u32>; MAP_WIDTH]; MAP_HEIGHT]) {
    for y in (0..MAP_HEIGHT).rev() {
        for x in 0..MAP_WIDTH {
            if let Some(value) = dijkstra_map[y][x] {
                print!("{:02} ", value);
            } else {
                print!("xx ");
            }
        }
        println!();
    }
    println!("--------------");
}

pub fn create_dijkstra_map(map: &Map, start_position: &GridPosition) -> [[Option<u32>; MAP_WIDTH]; MAP_HEIGHT] {
    let mut dijkstra_map: [[Option<u32>; MAP_WIDTH]; MAP_HEIGHT] = [[None; MAP_WIDTH]; MAP_HEIGHT];

    recurse_traverse_map(&mut dijkstra_map, map, start_position, 0);

    dijkstra_map
}

fn recurse_traverse_map(mut dijkstra_map: &mut [[Option<u32>; MAP_WIDTH]; MAP_HEIGHT], game_map: &Map, position: &GridPosition, distance: u32) {
    let x = position.x as usize;
    let y = position.y as usize;
    dijkstra_map[y][x] = Some(distance);
    let distance = distance + 1;

    for direction in DIRECTIONS.iter() {
        let new_x = position.x + direction.x;
        let new_y = position.y + direction.y;
        let new_position = GridPosition::new(new_x, new_y);
        if game_map.can_enter_tile(new_position) && (dijkstra_map[new_y as usize][new_x as usize].is_none() || dijkstra_map[new_y as usize][new_x as usize].is_some_and(|val| val > distance + 1)) {
            recurse_traverse_map(&mut dijkstra_map, &game_map, &new_position, distance);
        }
    }
}