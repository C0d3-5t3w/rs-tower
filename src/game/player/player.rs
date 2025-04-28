pub struct Player {
    health: u32,
    gold: u32,
    score: u32,
}

impl Player {
    pub fn new(health: u32, gold: u32) -> Self {
        Self {
            health,
            gold,
            score: 0,
        }
    }
    
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }
    
    pub fn add_gold(&mut self, amount: u32) {
        self.gold += amount;
    }
    
    pub fn spend_gold(&mut self, amount: u32) {
        if amount <= self.gold {
            self.gold -= amount;
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
    }
    
    pub fn get_health(&self) -> u32 {
        self.health
    }
    
    pub fn get_gold(&self) -> u32 {
        self.gold
    }
    
    pub fn get_score(&self) -> u32 {
        self.score
    }
    
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}
