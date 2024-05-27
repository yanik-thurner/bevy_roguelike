use std::time::Duration;
use crate::core::game_logic::grid::direction::Direction;
use bevy::ecs::system::EntityCommands;
use crate::prelude::*;

trait AnimationType {
    fn update(&mut self, original_transform: &Transform, delta: Duration) -> Transform;
}

pub struct AttackAnimation {
    timer: Timer,
    direction: Direction,
}

impl AnimationType for AttackAnimation {
    fn update(&mut self, original_transform: &Transform, delta: Duration) -> Transform {
        self.timer.tick(delta);
        let target = original_transform.clone().translation + Transform::from_xyz(self.direction.x as f32 * SPRITE_SIZE, self.direction.y as f32 * SPRITE_SIZE, 0.0).translation;
        original_transform.with_translation(original_transform.translation.lerp(target, self.timer.fraction()))
    }
}

pub struct WaitAnimation {
    timer: Timer,
}

impl AnimationType for WaitAnimation {
    fn update(&mut self, original_transform: &Transform, delta: Duration) -> Transform {
        self.timer.tick(delta);
        original_transform.with_scale(Vec3::new(1.0, 1.0 - 0.1 * self.timer.fraction(), 1.0))
    }
}

#[derive(Component)]
pub enum Animation {
    Attack(AttackAnimation),
    Wait(WaitAnimation),
}

impl Animation {
    pub fn new_attack_animation(direction: Direction) -> Self {
        Animation::Attack(AttackAnimation { timer: Timer::from_seconds(0.2, TimerMode::Once), direction })
    }

    pub fn new_wait_animation() -> Self{
        Animation::Wait(WaitAnimation{timer: Timer::from_seconds(0.1, TimerMode::Once)})
    }
    pub fn update(&mut self, current_transform: &mut Transform, original_transform: &Transform, delta: Duration) {
        match self {
            Animation::Attack(a) => *current_transform = a.update(original_transform, delta),
            Animation::Wait(a) => *current_transform = a.update(original_transform, delta)
        };
    }

    pub fn remove_if_finished(&self, commands: &mut EntityCommands) {
        match self {
            Animation::Attack(a) => { if a.timer.finished() { commands.remove::<Animation>(); } }
            Animation::Wait(a) => { if a.timer.finished() { commands.remove::<Animation>(); } }
        }
    }
}
