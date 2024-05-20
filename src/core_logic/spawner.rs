use crate::prelude::*;

pub fn spawn_random_monsters(mut commands: Commands, map: Res<MapResource>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, mut global_rng: GRng) {
    // map.0.rooms.iter().skip(1)
    //     .map(|r| (r.center(), match RngHelper::generate_from_range(global_rng.next_u32(), 0..10) {
    //         0..=8 => EnemyType::GOBLIN,
    //         _ => EnemyType::ORC
    //     }))
    //     .for_each(|(pos, enemy_type)| spawn_enemy(&mut commands, enemy_type, pos.into(), &asset_server, &mut texture_atlas_layouts));
}


struct EnemyData {
    pub hp: i32,
    #[allow(dead_code)]
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
        transform: Transform::from_xyz(0.0, 16.0, 4.0),
        ..default()
    })).id();

    let hp_bar = commands.spawn((EnemyHpBar, SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(SPRITE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 5.0),
        ..default()
    })).id();

    let mut cmds = commands.spawn((
        Enemy,
        Health::new(enemy_data.hp),
        Position(position),
        Attacker::new(),
        FieldOfView::new(6),
        ChasingPlayer,
        SpriteSheetBundle {
            texture: asset_server.load("dungeonfont.png"),
            transform: Transform { translation: Vec3::new(0.0, 0.0, 2.0), ..default() },
            visibility: Visibility::Hidden,
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: enemy_data.sprite_id as usize,
            },
            ..default()
        }));

    cmds.push_children(&[hp_root, hp_bar]);
}

#[allow(dead_code)]
pub fn spawn_monster(mut commands: Commands, enemy_type: EnemyType, position: GridPosition, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, mut grng: GRng) {
    spawn_enemy(&mut commands, enemy_type, position, &asset_server, &mut texture_atlas_layouts);
}

pub fn spawn_amulet_of_yala(mut commands: Commands, map: Res<MapResource>, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    // let layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    //
    // commands.spawn((Item, AmuletOfYala, Position::from_grid_position(map.spawn_amulet), SpriteSheetBundle {
    //     texture: asset_server.load("dungeonfont.png"),
    //     transform: Transform { translation: Vec3::new(0.0, 0.0, 2.0), ..default() },
    //     visibility: Visibility::Hidden,
    //     atlas: TextureAtlas {
    //         layout: texture_atlas_layout.clone(),
    //         index: 124,
    //     },
    //     ..default()
    // }));
}