use crate::prelude::*;

pub fn sync_cam_system(mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>, player_transform: Query<&Transform, (With<Player>, Without<PlayerCamera>)>) {
    let camera_translation = &mut camera.get_single_mut().unwrap().translation;
    let player_transform = player_transform.get_single().unwrap();
    camera_translation.x = player_transform.translation.x;
    camera_translation.y = player_transform.translation.y;
}