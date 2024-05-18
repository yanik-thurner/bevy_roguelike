use crate::components::map::Map;
use crate::prelude::*;

fn get_possible_next_positions(current_position: &GridPosition) -> [GridPosition; 4] {
    [GridPosition::new(current_position.x + 1, current_position.y),
        GridPosition::new(current_position.x - 1, current_position.y),
        GridPosition::new(current_position.x, current_position.y + 1),
        GridPosition::new(current_position.x, current_position.y - 1),
    ]
}


pub fn chase_player_system(map: Res<Map>, player_query: Query<(Entity, &Position), With<Player>>, enemy_query: Query<(Entity, &Position, &FieldOfView), (With<Enemy>, With<ChasingPlayer>, Without<Player>)>, mut ev_move: EventWriter<WantsToMoveEvent>, mut ev_attack: EventWriter<WantsToAttackEvent>) {
    let (player, player_position) = player_query.get_single().unwrap();

    let dijkstra_map = path_finding::create_dijkstra_map(&map, player_position.get());

    for (enemy, enemy_position, fov) in enemy_query.iter() {
        if !fov.visible_tiles.contains(player_position.get()) {
            continue;
        }
        let possible_next_positions = get_possible_next_positions(enemy_position.get());
        let (next_position, next_x, next_y) = possible_next_positions.iter()
            .map(|pos| (pos, pos.x as usize, pos.y as usize))
            .filter(|(pos, x, y)| map.in_bounds(*pos) && dijkstra_map[*y][*x].is_some())
            .min_by_key(|(_, x, y)| dijkstra_map[*y][*x]).unwrap();

        if let Some(distance) = dijkstra_map[next_y][next_x] {
            match distance {
                0 => { ev_attack.send(WantsToAttackEvent { attacker: enemy, victim: player }); }
                _ => { ev_move.send(WantsToMoveEvent { entity: enemy, destination: Position::from_grid_position(*next_position) }); }
            };
        }
    }
}

