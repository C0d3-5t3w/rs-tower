mod enemy1;
mod enemy2;

use ggez::GameResult;
use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect, Canvas};
use glam::Vec2;
use crate::game::map::Map;

pub use enemy1::Enemy1;
pub use enemy2::Enemy2;

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Normal,
    Fast,
    Boss,
}

pub struct Enemy {
    enemy_type: EnemyType,
    x: f32,
    y: f32,
    speed: f32,
    health: u32,
    max_health: u32,
    damage: u32,
    reward: u32,
    path_index: usize,
    spawn_delay: f32,
    active: bool,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, x: f32, y: f32, spawn_delay: f32) -> Self {
        match enemy_type {
            EnemyType::Normal => Enemy1::create(x, y, spawn_delay),
            EnemyType::Fast => Enemy2::create(x, y, spawn_delay),
            EnemyType::Boss => {
                let mut enemy = Enemy1::create(x, y, spawn_delay);
                enemy.health *= 3;
                enemy.max_health *= 3;
                enemy.damage *= 2;
                enemy.reward *= 3;
                enemy.speed *= 0.7;
                enemy
            }
        }
    }

    // Returns true if the enemy reached the end
    pub fn update(&mut self, dt: f32, map: &Map) -> bool {
        if self.spawn_delay > 0.0 {
            self.spawn_delay -= dt;
            return false;
        }
        
        if !self.active {
            self.active = true;
        }
        
        let path = map.get_path();
        if self.path_index >= path.len() - 1 {
            return true; // Reached end of path
        }
        
        let target = path[self.path_index + 1];
        let dx = target.0 - self.x;
        let dy = target.1 - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        
        if dist <= self.speed * dt {
            self.x = target.0;
            self.y = target.1;
            self.path_index += 1;
        } else {
            self.x += dx / dist * self.speed * dt;
            self.y += dy / dist * self.speed * dt;
        }
        
        // If health is zero, the enemy is dead but we'll let the game system
        // handle removal after collecting the reward
        if self.health == 0 {
            return false;
        }
        
        false
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut Canvas) -> GameResult {
        if !self.active {
            return Ok(());
        }
        
        // Draw enemy body
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(self.x, self.y),
            8.0,
            0.1,
            match self.enemy_type {
                EnemyType::Normal => [0.8, 0.2, 0.2, 1.0].into(),
                EnemyType::Fast => [0.2, 0.8, 0.2, 1.0].into(),
                EnemyType::Boss => [0.8, 0.4, 0.0, 1.0].into(),
            },
        )?;
        
        canvas.draw(&circle, DrawParam::default());
        
        // Draw health bar
        let health_pct = self.health as f32 / self.max_health as f32;
        let health_bar = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                self.x - 10.0,
                self.y - 15.0,
                20.0 * health_pct,
                3.0,
            ),
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;
        
        canvas.draw(&health_bar, DrawParam::default());
        
        Ok(())
    }
    
    pub fn take_damage(&mut self, damage: u32) -> bool {
        if damage >= self.health {
            self.health = 0;
            return true; // Enemy died
        }
        self.health -= damage;
        false
    }
    
    pub fn get_reward(&self) -> u32 {
        self.reward
    }
    
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn get_health(&self) -> u32 {
        self.health
    }
}
