use crate::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub move_cooldown: Timer,
}

impl Player {
    pub fn new() -> Self {
        Player {
            move_cooldown: Timer::from_seconds(0.15, TimerMode::Once)
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Npc;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Enemy;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(hp: i32) -> Self {
        Health {
            current: hp,
            max: hp,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ActorBundle {
    pub position: Position,
    pub health: Health,
    pub fov: FieldOfView,
    pub sprite: SpriteSheetBundle,
}

impl ActorBundle {
    pub fn new_player(position: GridPosition, assets: &Res<GameAssets>) -> (Player, ActorBundle) {
        println!("Spawning Player at ({:?})", &position);
        (Player::new(),
         ActorBundle {
             position: Position(position),
             health: Health::new(10),
             fov: FieldOfView::new(8),
             sprite: new_sprite(SPRITE_ID_PLAYER, true, &assets).attach_transform(&position, LAYER_PLAYER),
         })
    }

    pub fn new_enemy(position: GridPosition, assets: Res<GameAssets>) -> (Enemy, Npc, ActorBundle) {
        (Enemy, Npc,
         ActorBundle {
             position: Position(position),
             health: Health::new(2),
             fov: FieldOfView::new(7),
             sprite: new_sprite(SPRITE_ID_PLAYER, false, &assets).attach_transform(&position, LAYER_PLAYER),
         }
        )
    }
}