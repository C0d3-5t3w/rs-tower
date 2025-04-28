use super::{Tower, TowerType};

pub struct Tower2;

impl Tower2 {
    pub fn create(x: f32, y: f32) -> Tower {
        Tower {
            tower_type: TowerType::Advanced,
            x,
            y,
            range: 1500.0,
            damage: 25,
            fire_rate: 0.5, // 0.5 shots per second (slower but more powerful)
            fire_cooldown: 0.0,
            level: 1,
            cost: 100,
        }
    }
}
