use bevy::utils::HashSet;

pub use crate::prelude::*;



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
pub struct FieldOfViewComponent {
    pub visible_tiles: HashSet<GridPosition>,
    pub radius: usize,
    pub is_dirty: bool,
}

impl FieldOfViewComponent {
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