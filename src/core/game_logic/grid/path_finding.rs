use crate::{MAP_HEIGHT, MAP_WIDTH};
use crate::core::game_logic::grid::direction::Direction;
use crate::prelude::*;
use crate::prelude::encoding::YFirstEncoding;

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

pub fn create_dijkstra_map(enterable: &Grid<bool, YFirstEncoding>, start_position: &Position) -> Grid<Option<u32>, YFirstEncoding> {
    let mut dijkstra_map = Grid::new(enterable.get_dimensions(), enterable.get_min().to_tuple(), None);

    recurse_traverse_map(&mut dijkstra_map, &enterable, start_position, 0);

    dijkstra_map
}

fn recurse_traverse_map(mut dijkstra_map: &mut Grid<Option<u32>, YFirstEncoding>, enterable: &Grid<bool, YFirstEncoding>, position: &Position, distance: u32) {
    dijkstra_map[position] = Some(distance);
    let distance = distance + 1;

    for direction in Direction::ALL_DIRECTIONS.iter() {
        let new_position = *position + direction;
        if enterable[&new_position] && (dijkstra_map[&new_position].is_none() || dijkstra_map[&new_position].is_some_and(|val| val > distance + 1)) {
            recurse_traverse_map(&mut dijkstra_map, &enterable, &new_position, distance);
        }
    }
}