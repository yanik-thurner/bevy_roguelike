use bevy::render::view::RenderLayers;

use crate::core_logic::map::Map;
use crate::prelude::*;


pub fn spawn_player(mut commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let spawn = map.spawn
        ;
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((Player::new(), GridPosition { x: spawn.x, y: spawn.y }, Health { current: 10, max: 20 },
                    SpriteSheetBundle {
                        texture: asset_server.load("dungeonfont.png"),
                        transform: Transform { translation: Vec3::new(0.0, 0.0, 1.0), ..default() },
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 64,
                        },
                        ..default()
                    }));
}

pub fn spawn_random_monsters(mut commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    map.rooms.iter().skip(1)
        .map(|r| (r.center(), rand::random()))
        .for_each(|(pos, enemy_type)| spawn_enemy(&mut commands, enemy_type, pos.into(), &asset_server, &mut texture_atlas_layouts));
}

fn spawn_enemy(commands: &mut Commands, enemy_type: EnemyType, position: GridPosition, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let sprite_index = match enemy_type {
        EnemyType::ETTIN => 69,
        EnemyType::OGRE => 79,
        EnemyType::GOBLIN => 103,
        EnemyType::ORC => 111,
    };

    let hp_root = commands.spawn((EnemyHpRoot, SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(SPRITE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 3.0),
        ..default()
    })).id();

    let hp_bar = commands.spawn((EnemyHpBar, SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(SPRITE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 3.0),
        ..default()
    })).id();

    commands.spawn((
        Enemy,
        Health { current: 2, max: 2 },
        position,
        MovingRandomly,
        SpriteSheetBundle {
            texture: asset_server.load("dungeonfont.png"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 2.0), ..default() },
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: sprite_index,
            },
            ..default()
        })).push_children(&[hp_root, hp_bar]);
}

#[allow(dead_code)]
pub fn spawn_monster(mut commands: Commands, enemy_type: EnemyType, position: GridPosition, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    spawn_enemy(&mut commands, enemy_type, position, &asset_server, &mut texture_atlas_layouts);
}