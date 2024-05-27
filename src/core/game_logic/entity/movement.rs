use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Movement {
    MovingRandomly,
    ChasingPlayer,
}