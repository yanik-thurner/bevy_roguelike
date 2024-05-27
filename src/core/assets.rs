use crate::prelude::*;

pub const SPRITE_SIZE: f32 = 32.0;

pub const SPRITE_ID_WALL: usize = 35;
pub const SPRITE_ID_FLOOR: usize = 46;

pub const SPRITE_ID_PLAYER: usize = 64;
pub const SPRITE_ID_GOBLIN: usize = 103;
pub const SPRITE_ID_ORC: usize = 111;
pub const SPRITE_ID_OGRE: usize = 79;
pub const SPRITE_ID_ETTIN: usize = 69;

pub const SPRITE_ID_AMULET: usize = 124;

pub fn get_enemy_sprite_id_by_type(enemy_type: &EnemyType) -> usize {
    match enemy_type {
        EnemyType::GOBLIN => SPRITE_ID_GOBLIN,
        EnemyType::ORC => SPRITE_ID_ORC,
        EnemyType::OGRE => SPRITE_ID_OGRE,
        EnemyType::ETTIN => SPRITE_ID_ETTIN
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub sprite_sheet: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let sprite_sheet = asset_server.load("dungeonfont.png");
    let sprite_layout = TextureAtlasLayout::from_grid(Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 16, 16, None, None);
    let layout = texture_atlas_layouts.add(sprite_layout);
    commands.insert_resource(GameAssets {
        sprite_sheet,
        layout,
    });
}

pub struct SpriteSheetBundleWithoutTransform(SpriteSheetBundle);

impl SpriteSheetBundleWithoutTransform {
    pub fn attach_transform(&self, pos: &Position, layer: f32) -> SpriteSheetBundle {
        let mut ssb = self.0.clone();
        ssb.transform = Transform::from_xyz(pos.x as f32 * SPRITE_SIZE, pos.y as f32 * SPRITE_SIZE, layer);
        ssb
    }
}

pub fn new_sprite(index: usize, visible: bool, assets: &Res<GameAssets>) -> SpriteSheetBundleWithoutTransform {
    SpriteSheetBundleWithoutTransform(
        SpriteSheetBundle {
            texture: assets.sprite_sheet.clone(),
            visibility: match visible {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            },
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index,
            },
            ..default()
        })
}