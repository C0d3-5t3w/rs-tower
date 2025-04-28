use super::{Powerup, PowerupType};

pub struct Powerup2;

impl Powerup2 {
    pub fn create() -> Powerup {
        Powerup {
            powerup_type: PowerupType::GoldBoost,
            duration: 20.0, // 20 seconds
            active: false,
            cost: 150,
        }
    }
}
