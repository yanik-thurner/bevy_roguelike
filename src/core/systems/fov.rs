use crate::prelude::*;

pub fn fov_calculation(mut views: Query<(&Position, &mut FieldOfView)>, mut map: ResMut<Map>) {
    views.iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            println!("{:?}", pos);
            fov.update_visible_tiles(pos, &map);
            fov.visible_tiles.iter().for_each(|pos| { let _ = map.reveal(&pos); });

            fov.is_dirty = false;
        });
}

pub fn fov_revealing(mut commands: Commands,
                     mut query: Query<(Entity, &mut Visibility, &Position, Option<&Enemy>), Without<PlayerCamera>>,
                     mut revealed: Query<&mut Sprite, (With<Revealed>, Or<(With<Wall>, With<Floor>)>)>,
                     player: Query<&FieldOfView, With<Player>>) {
    for (entity, mut visibility, position, enemy) in query.iter_mut() {
        if let Some(fov) = player.get_single().ok() {
            if fov.visible_tiles.contains(&position) {
                *visibility = Visibility::Visible;
                commands.entity(entity).insert(Revealed);
            } else if enemy.is_some() {
                *visibility = Visibility::Hidden;
            }

            if let Ok(mut sprite) = revealed.get_mut(entity) {
                match fov.visible_tiles.contains(&position) {
                    true => { sprite.color.set_a(1.0); }
                    false => { sprite.color.set_a(0.5); }
                }
            }
        }
    }
}