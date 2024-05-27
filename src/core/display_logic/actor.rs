use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Npc;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    pub input_cooldown: InputCooldown,
    pub position: Position,
    pub health: Health,
    pub fov: FieldOfView,
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    marker_enemy: Enemy,
    marker_npc: Npc,
    pub entropy_component: EntropyComponent<WyRand>,
    pub position: Position,
    pub movement: Movement,
    pub health: Health,
    pub fov: FieldOfView,
    pub sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(position: Position, assets: &Res<GameAssets>) -> Self {
        println!("Spawning Player at ({:?})", &position);
        PlayerBundle {
            marker: Player,
            input_cooldown: InputCooldown::new(),
            position,
            health: Health::new(100),
            fov: FieldOfView::new(8),
            sprite: new_sprite(SPRITE_ID_PLAYER, true, &assets).attach_transform(&position, LAYER_PLAYER),
        }
    }
}


impl EnemyBundle {
    pub fn new(entropy_component: EntropyComponent<WyRand>, position: Position, movement: Movement, enemy_type: EnemyType, assets: &Res<GameAssets>) -> Self {
        let enemy_data = EnemyStats::get_by_type(&enemy_type);
        EnemyBundle {
            marker_enemy: Enemy,
            marker_npc: Npc,
            entropy_component,
            position,
            movement,
            health: enemy_data.health,
            fov: FieldOfView::new(enemy_data.fov),
            sprite: new_sprite(get_enemy_sprite_id_by_type(&enemy_type), false, &assets).attach_transform(&position, LAYER_PLAYER),
        }
    }
}