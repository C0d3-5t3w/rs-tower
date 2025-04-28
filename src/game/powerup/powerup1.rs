use super::{Powerup, PowerupType};

pub struct Powerup1;

impl Powerup1 {
    pub fn create() -> Powerup {
        Powerup {
            powerup_type: PowerupType::DamageBoost,
            duration: 10.0, // 10 seconds
            active: false,
            cost: 100,
        }
    }
}
