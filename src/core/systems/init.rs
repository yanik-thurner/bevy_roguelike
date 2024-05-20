use bevy::input::keyboard::Key::Camera;
use crate::core::actor::ActorBundle;
use crate::prelude::*;


pub fn init_cam(mut commands: Commands, mut existing_camera: Query<Entity, With<PlayerCamera>>, player: Query<&PositionComponent, With<PlayerComponent>>) {
    let camera = if existing_camera.is_empty() {
        commands.spawn((PlayerCamera, Camera2dBundle::default())).id()
    } else {
        existing_camera.get_single_mut().unwrap()
    };

    let mut new_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    if let Some(player) = player.get_single().ok() {
        println!("Centering Cam at {:?}", player.grid_to_transform());
        new_transform.translation.x = player.grid_to_transform().translation.x;
        new_transform.translation.y = player.grid_to_transform().translation.y;
    }
    commands.entity(camera).insert(new_transform);
}

pub fn init_map(mut commands: Commands, assets: Res<GameAssets>, mut rng: GRng) {
    let map_builder = MapBuilder::build(20, 20, 20, rng.next_u64());

    spawn_tiles(&mut commands, &map_builder.map, &assets);

    commands.insert_resource(MapResource(map_builder.map));
    commands.spawn(ActorBundle::new_player(map_builder.spawn_player, &assets));
}

fn spawn_tiles(mut commands: &mut Commands, map: &Map, assets: &Res<GameAssets>) {
    map.iter_tiles().for_each(|tile| {
        match tile.tile_type {
            TileType::Floor => {
                commands.spawn(StructureBundle::new_floor(tile.get_pos().clone(), &assets));
            }
            TileType::Wall => {
                commands.spawn(StructureBundle::new_wall(tile.get_pos().clone(), &assets));
            }
        }
    })
}

fn spawn_amulet(mut commands: &mut Commands) {}

fn spawn_enemies(mut commands: &mut Commands) {}