use bevy::utils::HashMap;
use crate::prelude::*;



pub fn process_movement_events(mut commands: Commands,
                               mut ev_wants_to_move: EventReader<WantsToMoveEvent>,
                               map: Res<Map>,
                               entities: Query<(Entity, &Position, &FieldOfView), Or<(With<Enemy>, With<Player>)>>) {
    if ev_wants_to_move.is_empty() {
        return;
    }

    let mut made_movements = HashMap::new();
    for ev in ev_wants_to_move.read() {
        let at_this_position = entities.iter().filter(|(entity, pos, _fov)| **pos == ev.destination && ev.entity != *entity && made_movements.values().all(|moved_from: &&Position| **moved_from != ev.destination)).count();

        if map.can_enter_tile(&ev.destination) && !made_movements.contains_key(&ev.destination) && at_this_position == 0 {
            commands.entity(ev.entity).insert(ev.destination);
            commands.entity(ev.entity).insert(entities.get(ev.entity).unwrap().2.clone_dirty());
            let current_position = entities.get(ev.entity).unwrap().1;
            made_movements.insert(ev.destination, current_position);
        }
    }
}