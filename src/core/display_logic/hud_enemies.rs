use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct EnemyHpRoot;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct EnemyHpBar;
pub fn setup_enemy_health_bar(commands: &mut Commands, enemy: Entity) {
    let hp_root = commands.spawn((EnemyHpRoot, SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(SPRITE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 4.0),
        ..default()
    })).id();

    let hp_bar = commands.spawn((EnemyHpBar, SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(SPRITE_SIZE, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 5.0),
        ..default()
    })).id();

    commands.entity(enemy).push_children(&[hp_root, hp_bar]);
}

pub fn update_enemy_hud(health_query: Query<&Health, With<Enemy>>, mut health_bar_query: Query<(&mut Transform, &Parent), With<EnemyHpBar>>) {
    for (mut transform, parent) in health_bar_query.iter_mut() {
        let target_health = health_query.get(parent.get());
        if let Some(target_health) = target_health.ok() {
            transform.scale.x = target_health.current as f32 / target_health.max as f32;
            transform.translation.x = (SPRITE_SIZE / 2.0 * transform.scale.x) - (SPRITE_SIZE / 2.0);
        }
    }
}