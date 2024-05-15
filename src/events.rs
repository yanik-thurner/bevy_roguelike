use bevy::prelude::{Entity, Event};

use crate::prelude::GridPosition;

#[derive(Event)]
pub struct WantsToMoveEvent {
    pub entity: Entity,
    pub destination: GridPosition,
}

#[derive(Event)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
