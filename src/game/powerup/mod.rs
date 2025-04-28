mod powerup1;
mod powerup2;

pub use powerup1::Powerup1;
pub use powerup2::Powerup2;

#[derive(Debug, Clone, Copy)]
pub enum PowerupType {
    DamageBoost,
    GoldBoost,
}

pub struct Powerup {
    powerup_type: PowerupType,
    duration: f32,
    active: bool,
    cost: u32,
}

impl Powerup {
    pub fn new(powerup_type: PowerupType) -> Self {
        match powerup_type {
            PowerupType::DamageBoost => Powerup1::create(),
            PowerupType::GoldBoost => Powerup2::create(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }

        self.duration -= dt;
        if self.duration <= 0.0 {
            self.active = false;
            return false;
        }

        true
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }
    
    pub fn get_duration(&self) -> f32 {
        self.duration
    }
    
    pub fn get_type(&self) -> PowerupType {
        self.powerup_type
    }
}
