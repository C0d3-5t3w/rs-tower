mod tower1;
mod tower2;

use ggez::graphics::{DrawMode, DrawParam, Mesh};
use ggez::GameResult;
use glam::Vec2;

pub use tower1::Tower1;
pub use tower2::Tower2;

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    Basic,
    Advanced,
}

pub struct Tower {
    tower_type: TowerType,
    x: f32,
    y: f32,
    range: f32,
    damage: u32,
    fire_rate: f32, // shots per second
    fire_cooldown: f32,
    level: u32,
    cost: u32,
}

impl Tower {
    pub fn new(tower_type: TowerType, x: f32, y: f32) -> Self {
        match tower_type {
            TowerType::Basic => Tower1::create(x, y),
            TowerType::Advanced => Tower2::create(x, y),
        }
    }

    pub fn update(&mut self, dt: f32, enemies: &[(f32, f32)]) -> Option<(f32, f32)> {
        self.fire_cooldown -= dt;
        
        // Find closest enemy in range
        if self.fire_cooldown <= 0.0 {
            if let Some(target) = self.find_target(enemies) {
                self.fire_cooldown = 1.0 / self.fire_rate;
                return Some(target);
            }
        }
        
        None
    }

    fn find_target(&self, enemies: &[(f32, f32)]) -> Option<(f32, f32)> {
        enemies.iter()
            .filter(|(ex, ey)| {
                let dx = ex - self.x;
                let dy = ey - self.y;
                dx*dx + dy*dy <= self.range*self.range
            })
            .copied()
            .next()
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        // Draw the tower base
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(self.x, self.y),
            10.0,
            0.1,
            match self.tower_type {
                TowerType::Basic => [0.2, 0.6, 0.8, 1.0].into(),
                TowerType::Advanced => [0.8, 0.2, 0.8, 1.0].into(),
            },
        )?;
        
        canvas.draw(&circle, DrawParam::default());
        
        // Draw range indicator when selected (could be toggled with a flag)
        if self.fire_cooldown < 0.1 {
            // Optional: Draw shooting indicator when tower just fired
            let shot_indicator = Mesh::new_circle(
                ctx,
                DrawMode::stroke(1.0),
                Vec2::new(self.x, self.y),
                15.0,
                0.1,
                [1.0, 1.0, 0.0, 0.5].into(),
            )?;
            canvas.draw(&shot_indicator, DrawParam::default());
        }
        
        Ok(())
    }

    pub fn upgrade(&mut self) -> u32 {
        let upgrade_cost = self.cost / 2;
        self.level += 1;
        self.damage += self.damage / 2;
        self.range += 10.0;
        self.fire_rate *= 1.2;  // 20% faster firing
        upgrade_cost
    }

    pub fn get_sell_value(&self) -> u32 {
        self.cost / 2 + (self.level - 1) * (self.cost / 4)
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_range(&self) -> f32 {
        self.range
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_tower_type(&self) -> TowerType {
        self.tower_type
    }
}

impl TowerType {
    pub fn cost(&self) -> u32 {
        match self {
            TowerType::Basic => 50,
            TowerType::Advanced => 100,
        }
    }
}
