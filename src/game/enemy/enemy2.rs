use super::{Enemy, EnemyType};

pub struct Enemy2;

impl Enemy2 {
    pub fn create(x: f32, y: f32, spawn_delay: f32) -> Enemy {
        Enemy {
            enemy_type: EnemyType::Fast,
            x,
            y,
            speed: 100.0, // Faster
            health: 50,   // Less health
            max_health: 50,
            damage: 5,    // Less damage
            reward: 15,
            path_index: 0,
            spawn_delay,
            active: false,
        }
    }
}
