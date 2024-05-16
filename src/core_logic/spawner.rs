use rand::Rng;

use crate::core_logic::map::Map;
use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let spawn = map.spawn
        ;
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Player::new(),
        GridPosition { x: spawn.x, y: spawn.y },
        Health { before: 10, current: 10, max: 10 },
        Attacker::new(),
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


struct EnemyData {
    pub hp: i32,
    pub name: String,
    pub sprite_id: i32,
}

impl EnemyData {
    pub fn generate_by_type(enemy_type: &EnemyType) -> Self {
        match enemy_type {
            EnemyType::ETTIN => todo!(), //69
            EnemyType::OGRE => todo!(), //79
            EnemyType::GOBLIN => EnemyData::generate_goblin(),
            EnemyType::ORC => EnemyData::generate_orc(),
        }
    }
    pub fn generate_goblin() -> Self {
        EnemyData {
            hp: 1,
            name: "Goblin".into(),
            sprite_id: 103,
        }
    }

    pub fn generate_orc() -> Self {
        EnemyData {
            hp: 2,
            name: "Orc".into(),
            sprite_id: 111,
        }
    }
}

fn spawn_enemy(commands: &mut Commands, enemy_type: EnemyType, position: GridPosition, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let enemy_data = EnemyData::generate_by_type(&enemy_type);

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

    let mut cmds = commands.spawn((
        Enemy,
        Health { before: enemy_data.hp, current: enemy_data.hp, max: enemy_data.hp },
        position,
        Attacker::new(),
        SpriteSheetBundle {
            texture: asset_server.load("dungeonfont.p\
            ng"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 2.0), ..default() },
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: enemy_data.sprite_id as usize,
            },
            ..default()
        }));

    match rand::thread_rng().gen_range(0..10) {
        0..=8 => cmds.insert(ChasingPlayer),
        _ => cmds.insert(MovingRandomly),
    };

    cmds.push_children(&[hp_root, hp_bar]);
}

#[allow(dead_code)]
pub fn spawn_monster(mut commands: Commands, enemy_type: EnemyType, position: GridPosition, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    spawn_enemy(&mut commands, enemy_type, position, &asset_server, &mut texture_atlas_layouts);
}