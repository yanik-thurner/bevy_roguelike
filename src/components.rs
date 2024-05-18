use bevy::utils::HashSet;

pub use crate::prelude::*;



#[derive(Component, Clone, Debug, PartialEq)]
pub struct Player {
    pub move_cooldown: Timer,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

pub enum EnemyType {
    ETTIN,
    OGRE,
    ORC,
    GOBLIN,
}

impl Player {
    pub fn new() -> Self {
        Player {
            move_cooldown: Timer::from_seconds(0.15, TimerMode::Once)
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera;


#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub before: i32,
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct EnemyHpRoot;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct EnemyHpBar;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Attacker {
    pub cooldown: Timer,
    pub attack: Option<Entity>,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Attack;

impl Attacker {
    pub fn new() -> Self {
        Attacker {
            cooldown: Timer::from_seconds(0.1, TimerMode::Once),
            attack: None,
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Item;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<GridPosition>,
    pub radius: usize,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: usize) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Revealed;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct WallOrFloor;