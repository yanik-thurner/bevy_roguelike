use bevy::prelude::{Entity, Event};
use crate::prelude::*;


#[derive(Event)]
pub struct WantsToMoveEvent {
    pub entity: Entity,
    pub destination: PositionComponent,
}

#[derive(Event)]
pub struct WantsToAttackEvent {
    pub attacker: Entity,
    pub victim: Entity,
}
