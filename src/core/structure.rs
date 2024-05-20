use bevy::reflect::erased_serde::__private::serde::__private::de::Content::Str;
use crate::prelude::*;


#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wall;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Floor;

#[derive(Bundle, Clone)]
pub struct StructureBundle {
    pub position: PositionComponent,
    pub sprite: SpriteSheetBundle,
}

impl StructureBundle {
    pub fn new_floor(position: GridPosition, assets: &Res<GameAssets>) -> (Floor, Self) {
        (Floor,
         StructureBundle {
             position: PositionComponent(position),
             sprite: new_sprite(SPRITE_ID_FLOOR, true, assets).attach_transform(&position, LAYER_FLOOR),
         }
        )
    }

    pub fn new_wall(position: GridPosition, assets: &Res<GameAssets>) -> (Wall, Self) {
        (Wall,
         StructureBundle {
             position: PositionComponent(position),
             sprite: new_sprite(SPRITE_ID_WALL, true, assets).attach_transform(&position, LAYER_WALL),
         }
        )
    }
}