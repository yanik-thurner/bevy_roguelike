use bevy::render::view::RenderLayers;

use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let spawn = map.spawn
        ;
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((Player::new(), GridPosition { x: spawn.x, y: spawn.y }, RenderBundle {
        sprite: SpriteSheetBundle {
            texture: asset_server.load("dungeonfont.png"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 1.0), ..default() },
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 64,
            },
            ..default()
        },
        layer: RenderLayers::layer(1),
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

    commands.spawn((Enemy, position, MovingRandomly, RenderBundle {
        sprite: SpriteSheetBundle {
            texture: asset_server.load("dungeonfont.png"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 1.0), ..default() },
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: sprite_index,
            },
            ..default()
        },
        layer: RenderLayers::layer(1),
    }));
}

#[allow(dead_code)]
pub fn spawn_monster(mut commands: Commands, enemy_type: EnemyType, position: GridPosition, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    spawn_enemy(&mut commands, enemy_type, position, &asset_server, &mut texture_atlas_layouts);
}