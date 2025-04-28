use super::{Tower, TowerType};

pub struct Tower1;

impl Tower1 {
    pub fn create(x: f32, y: f32) -> Tower {
        Tower {
            tower_type: TowerType::Basic,
            x,
            y,
            range: 1000.0,
            damage: 10,
            fire_rate: 1.0, // 1 shot per second
            fire_cooldown: 0.0,
            level: 1,
            cost: 50,
        }
    }
}
