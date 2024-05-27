use bevy::prelude::{Entity, Event};

use crate::core::game_logic::grid::direction::Direction;
use crate::prelude::*;

#[derive(Event)]
pub struct WantsToMoveEvent {
    pub entity: Entity,
    pub destination: Position,
    pub direction: Direction
}

#[derive(Event)]
pub struct WantsToAttackEvent {
    pub attacker: Entity,
    pub victim: Entity,
    pub direction: Direction
}
