use crate::prelude::*;

pub fn update_enemy_hud(health_query: Query<&HealthComponent, With<EnemyComponent>>, mut health_bar_query: Query<(&mut Transform, &Parent), With<EnemyHpBar>>) {
    for (mut transform, parent) in health_bar_query.iter_mut() {
        let target_health = health_query.get(parent.get());
        if let Some(target_health) = target_health.ok() {
            transform.scale.x = target_health.0.current as f32 / target_health.0.max as f32;
            transform.translation.x = (SPRITE_SIZE / 2.0 * transform.scale.x) - (SPRITE_SIZE / 2.0);
        }
    }
}