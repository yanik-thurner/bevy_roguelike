use crate::core::game_logic::grid::direction::Direction;
use crate::prelude::*;
use crate::prelude::encoding::YFirstEncoding;

fn get_direction_towards_player(mover_position: &Position, player_position: &Position, enterable: &Grid<bool, YFirstEncoding>) -> Direction {
    let dijkstra_map = create_dijkstra_map(enterable, &player_position);

    // if !fov.visible_tiles.contains(player_position) {
    //     continue;
    // }
    let (next_direction, next_position) = mover_position.get_adjacent_positions().into_iter()
        .filter(|(_, pos)| enterable.contains_position(pos) && dijkstra_map[pos].is_some())
        .min_by_key(|(_, pos)| dijkstra_map[pos]).unwrap_or((Direction::NONE, *mover_position));

    return if next_direction != Direction::NONE && dijkstra_map[&next_position].is_some() {
        next_direction.clone()
    } else {
        Direction::NONE
    };
}

pub fn generate_action_events(mut actors: Query<(Entity, &Position, &Movement, &mut EntropyComponent<WyRand>)>,
                              targets: Query<(Entity, &Position), With<Health>>,
                              player: Query<&Position, With<Player>>,
                              map: Res<Map>,
                              mut ev_wants_to_move: EventWriter<WantsToMoveEvent>,
                              mut ev_wants_to_attack: EventWriter<WantsToAttackEvent>) {
    if player.is_empty() {
        return;
    }

    let player_position = player.get_single().unwrap();

    let mut enterable = map.tiles.map(|t| *t == TileType::Floor);
    actors.iter().for_each(|(_, pos, _, _)| enterable[pos] = false);
    enterable[player_position] = true;

    for (mover_entity, mover_position, mover_strategy, mut entropy) in actors.iter_mut() {
        let direction = match mover_strategy {
            Movement::MovingRandomly => Direction::ALL_DIRECTIONS_INCLUDING_NONE[RandomUtil::pick_from_range(entropy.next_u32(), 0..9) as usize],
            Movement::ChasingPlayer => get_direction_towards_player(mover_position, player_position, &enterable)
        };

        let target = targets
            .iter()
            .filter(|(potential_target, potential_target_position)| **potential_target_position == (*mover_position + direction) && *potential_target != mover_entity)
            .map(|(target, _)| target)
            .next();


        if let Some(enemy) = target {
            ev_wants_to_attack.send(WantsToAttackEvent { attacker: mover_entity, victim: enemy, direction });
        } else {
            ev_wants_to_move.send(WantsToMoveEvent { entity: mover_entity, destination: *mover_position + direction, direction });
        }
    }
}