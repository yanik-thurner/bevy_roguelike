use std::collections::HashMap;
use std::fmt::format;
use bevy::utils::petgraph::visit::Walker;
use crate::components::map::{Map, MAP_HEIGHT};
use crate::prelude::*;
use crate::prelude::map::MAP_WIDTH;

const DIRECTIONS: [GridPosition; 4] = [GridPosition::NORTH, GridPosition::EAST, GridPosition::SOUTH, GridPosition::WEST];

fn get_possible_next_positions(current_position: &GridPosition) -> [GridPosition; 4] {
    [GridPosition::new(current_position.x + 1, current_position.y),
        GridPosition::new(current_position.x - 1, current_position.y),
        GridPosition::new(current_position.x, current_position.y + 1),
        GridPosition::new(current_position.x, current_position.y - 1),
    ]
}

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

pub fn chase_player_system(map: Res<Map>, player_query: Query<(Entity, &GridPosition), With<Player>>, enemy_query: Query<(Entity, &GridPosition), (With<Enemy>, With<ChasingPlayer>, Without<Player>)>, mut ev_move: EventWriter<WantsToMoveEvent>, mut ev_attack: EventWriter<WantsToAttackEvent>) {
    let mut dijkstra_map: [[Option<u32>; MAP_WIDTH]; MAP_HEIGHT] = [[None; MAP_WIDTH]; MAP_HEIGHT];
    let (player, player_position) = player_query.get_single().unwrap();

    recurse_traverse_map(&mut dijkstra_map, &map, &player_position, 0);
    for (enemy, enemy_position) in enemy_query.iter() {
        let possible_next_positions = get_possible_next_positions(enemy_position);
        let (next_position, next_x, next_y) = possible_next_positions.iter()
            .map(|pos| (pos, pos.x as usize, pos.y as usize))
            .filter(|(pos, x, y)| map.in_bounds(**pos) && dijkstra_map[*y][*x].is_some())
            .min_by_key(|(_, x, y)| dijkstra_map[*y][*x]).unwrap();

        if let Some(distance) = dijkstra_map[next_y][next_x] {
            match distance {
                0 => { ev_attack.send(WantsToAttackEvent { attacker: enemy, victim: player }); }
                _ => { ev_move.send(WantsToMoveEvent { entity: enemy, destination: *next_position }); }
            };
        }
    }
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