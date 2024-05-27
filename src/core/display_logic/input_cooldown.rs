use crate::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct InputCooldown {
    pub timer: Timer,
}

impl InputCooldown{
    pub fn new() -> Self{
        InputCooldown {
            timer: Timer::from_seconds(0.15, TimerMode::Once)
        }
    }
}
