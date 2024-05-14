use bevy::render::view::RenderLayers;
use rand::{distributions::{Distribution, Standard}, Rng, thread_rng};
use rand::prelude::IndexedRandom;

pub use crate::prelude::*;

#[derive(Bundle)]
pub struct RenderBundle {
    pub sprite: SpriteSheetBundle,
    pub layer: RenderLayers,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GridPosition
{
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub const ZERO: GridPosition = GridPosition { x: 0, y: 0 };

    pub const NORTH: GridPosition = GridPosition { x: 0, y: 1 };
    pub const SOUTH: GridPosition = GridPosition { x: 0, y: -1 };
    pub const EAST: GridPosition = GridPosition { x: 1, y: 0 };
    pub const WEST: GridPosition = GridPosition { x: -1, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition {
            x,
            y,
        }
    }

    pub fn random_direction(include_no_movement: bool) -> GridPosition {
        let options = if include_no_movement { vec![Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST] } else { vec![Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST, Self::ZERO] };
        let mut rng = thread_rng();

        *(options.choose(&mut rng).unwrap())
    }
}

impl From<Vec2> for GridPosition {
    fn from(value: Vec2) -> Self {
        GridPosition {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl Into<Vec2> for GridPosition {
    fn into(self) -> Vec2 {
        return Vec2::new(self.x as f32, self.y as f32);
    }
}

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

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        match rng.gen_range(0..=3) {
            0 => EnemyType::ETTIN,
            1 => EnemyType::OGRE,
            2 => EnemyType::ORC,
            _ => EnemyType::GOBLIN
        }
    }
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

#[derive(Component)]
pub struct MovingRandomly;


