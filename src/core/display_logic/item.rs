use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AmuletOfYala;


#[derive(Bundle)]
pub struct ItemBundle {
    pub position: Position,
    pub sprite: SpriteSheetBundle,
}

impl ItemBundle {
    pub fn new_amulet(position: Position, assets: &Res<GameAssets>) -> (AmuletOfYala, Self) {
        (AmuletOfYala,
         ItemBundle {
             position,
             sprite: new_sprite(SPRITE_ID_AMULET, false, assets).attach_transform(&position, LAYER_PLAYER),
         })
    }
}

