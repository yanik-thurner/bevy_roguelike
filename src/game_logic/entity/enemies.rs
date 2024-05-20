use super::super::prelude::*;

pub enum EnemyType {
    ETTIN,
    OGRE,
    ORC,
    GOBLIN,
}

pub struct EnemyStats {
    enemy_type: EnemyType,
    pub health: Health,
    pub fov: usize,
}

impl EnemyStats {
    pub fn get_type(&self) -> &EnemyType {
        &self.enemy_type
    }
    pub fn get_by_type(enemy_type: &EnemyType) -> Self {
        match enemy_type {
            EnemyType::GOBLIN => Self::get_goblin(),
            EnemyType::ORC => Self::get_orc(),
            EnemyType::OGRE => Self::get_ogre(),
            EnemyType::ETTIN => Self::get_ettin(),
        }
    }

    fn get_goblin() -> Self {
        EnemyStats {
            enemy_type: EnemyType::GOBLIN,
            health: Health::new(1),
            fov: 10,
        }
    }

    fn get_orc() -> Self {
        EnemyStats {
            enemy_type: EnemyType::ORC,
            health: Health::new(2),
            fov: 6,
        }
    }

    fn get_ogre() -> Self {
        EnemyStats {
            enemy_type: EnemyType::OGRE,
            health: Health::new(4),
            fov: 7,
        }
    }

    fn get_ettin() -> Self {
        EnemyStats {
            enemy_type: EnemyType::ETTIN,
            health: Health::new(10),
            fov: 8,
        }
    }
}