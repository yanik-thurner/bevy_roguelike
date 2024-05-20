use crate::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct PlayerComponent {
    pub move_cooldown: Timer,
}

impl PlayerComponent {
    pub fn new() -> Self {
        PlayerComponent {
            move_cooldown: Timer::from_seconds(0.15, TimerMode::Once)
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Npc;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyComponent;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HealthComponent(pub Health);

#[derive(Bundle, Clone)]
pub struct ActorBundle {
    pub position: PositionComponent,
    pub health: HealthComponent,
    pub fov: FieldOfViewComponent,
    pub sprite: SpriteSheetBundle,
}

impl ActorBundle {
    pub fn new_player(position: GridPosition, assets: &Res<GameAssets>) -> (PlayerComponent, ActorBundle) {
        println!("Spawning Player at ({:?})", &position);
        (PlayerComponent::new(),
         ActorBundle {
             position: PositionComponent(position),
             health: HealthComponent(Health::new(10)),
             fov: FieldOfViewComponent::new(8),
             sprite: new_sprite(SPRITE_ID_PLAYER, true, &assets).attach_transform(&position, LAYER_PLAYER),
         })
    }

    pub fn new_enemy(position: GridPosition, assets: Res<GameAssets>) -> (EnemyComponent, Npc, ActorBundle) {
        (EnemyComponent, Npc,
         ActorBundle {
             position: PositionComponent(position),
             health: HealthComponent(Health::new(2)),
             fov: FieldOfViewComponent::new(7),
             sprite: new_sprite(SPRITE_ID_PLAYER, false, &assets).attach_transform(&position, LAYER_PLAYER),
         }
        )
    }
}