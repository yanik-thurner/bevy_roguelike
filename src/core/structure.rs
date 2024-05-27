use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wall;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Floor;

#[derive(Bundle, Clone)]
pub struct StructureBundle {
    pub position: Position,
    pub sprite: SpriteSheetBundle,
}

impl StructureBundle {
    pub fn new_floor(position: Position, assets: &Res<GameAssets>) -> (Floor, Self) {
        (Floor,
         StructureBundle {
             position,
             sprite: new_sprite(SPRITE_ID_FLOOR, false, assets).attach_transform(&position, LAYER_FLOOR),
         }
        )
    }

    pub fn new_wall(position: Position, assets: &Res<GameAssets>) -> (Wall, Self) {
        (Wall,
         StructureBundle {
             position,
             sprite: new_sprite(SPRITE_ID_WALL, false, assets).attach_transform(&position, LAYER_WALL),
         }
        )
    }
}