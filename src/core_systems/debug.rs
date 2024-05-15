use crate::prelude::*;

pub fn debug_system(query: Query<&Transform, (With<EnemyHpBar>)>) {}