use std::collections::HashSet;

use crate::prelude::*;

const ENEMIES_PER_TILE: f32 = 0.0075;
const ENEMY_SPAWN_TRIES: usize = 1_000;

pub fn init_cam(mut commands: Commands, mut existing_camera: Query<Entity, With<PlayerCamera>>, player: Query<&Position, With<Player>>) {
    let camera = if existing_camera.is_empty() {
        commands.spawn((PlayerCamera, Camera2dBundle::default())).id()
    } else {
        existing_camera.get_single_mut().unwrap()
    };

    let mut new_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    if let Some(player) = player.get_single().ok() {
        println!("Centering Cam at {:?}", player);
        new_transform.translation.x = player.x as f32 * SPRITE_SIZE;
        new_transform.translation.y = player.y as f32 * SPRITE_SIZE;
    }
    commands.entity(camera).insert(new_transform);
}

pub fn init_hud(mut commands: Commands) {
    setup_hud(&mut commands);
}

pub fn init_map(mut commands: Commands, assets: Res<GameAssets>, mut rng: GRng) {
    let map_builder = MapBuilder::build(20, 20, 20, rng.next_u64());

    spawn_tiles(&mut commands, &map_builder.map, &assets);

    commands.insert_resource(map_builder.map);
    commands.spawn(PlayerBundle::new(map_builder.spawn_player, &assets));
    commands.spawn(ItemBundle::new_amulet(map_builder.spawn_amulet, &assets));
}

pub fn init_enemies(mut commands: Commands, map: Res<Map>, assets: Res<GameAssets>, mut rng: GRng) {
    let (map_width, map_height) = map.get_dimensions();
    let num_enemies = ((map_width * map_height) as f32 * ENEMIES_PER_TILE).round() as usize;

    let mut count = 0;
    let mut spawned_enemies = HashSet::new();
    while spawned_enemies.len() < num_enemies && count < ENEMY_SPAWN_TRIES {
        let spawn = RandomUtil::pick_position_in_rect(rng.next_u32(), map.get_rect());
        if map.can_enter_tile(&spawn) && !spawned_enemies.contains(&spawn) {
            let enemy_type = match RandomUtil::pick_from_range(rng.next_u32(), 0..100)
            {
                0..=70 => EnemyType::GOBLIN,
                71..=85 => EnemyType::ORC,
                86..=95 => EnemyType::OGRE,
                _ => EnemyType::ETTIN,
            };
            let movement_type = match RandomUtil::true_or_false(rng.next_u32(), 0.8) {
                true => Movement::ChasingPlayer,
                false => Movement::MovingRandomly
            };
            println!("Spawning {:?} @ {:?}", &enemy_type, spawn);
            let enemy = commands.spawn(EnemyBundle::new(rng.fork_rng(), spawn, movement_type, enemy_type, &assets)).id();
            setup_enemy_health_bar(&mut commands, enemy);
            spawned_enemies.insert(spawn);
        }
        count += 1;
    }
}

fn spawn_tiles(commands: &mut Commands, map: &Map, assets: &Res<GameAssets>) {
    map.tiles.iter_cells().for_each(|(position, tile)| {
        match tile {
            TileType::Floor => {
                commands.spawn(StructureBundle::new_floor(*position, &assets));
            }
            TileType::Wall => {
                commands.spawn(StructureBundle::new_wall(*position, &assets));
            }
        }
    })
}