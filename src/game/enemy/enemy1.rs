use super::{Enemy, EnemyType};

pub struct Enemy1;

impl Enemy1 {
    pub fn create(x: f32, y: f32, spawn_delay: f32) -> Enemy {
        Enemy {
            enemy_type: EnemyType::Normal,
            x,
            y,
            speed: 50.0,
            health: 100,
            max_health: 100,
            damage: 10,
            reward: 20,
            path_index: 0,
            spawn_delay,
            active: false,
        }
    }
}
