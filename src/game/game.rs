use ggez::{Context, GameResult, graphics};
use hecs::World;
use glam::Vec2;

use crate::game::enemy::{Enemy, EnemyType};
use crate::game::map::{Map, MapType};
use crate::game::player::Player;
use crate::game::tower::{Tower, TowerType};

pub struct Game {
    world: World,
    player: Player,
    current_map: MapType,
    wave: u32,
    game_time: f32,
    wave_cooldown: f32,
    game_state: GameState,
    projectiles: Vec<Projectile>, // Add projectiles storage
}

// Adding a Projectile struct to visualize tower shots
pub struct Projectile {
    start: Vec2,
    target: Vec2,
    time_alive: f32,
    max_lifetime: f32,
    color: [f32; 4],
}

#[derive(Debug, Clone)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
    Victory,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let world = World::new();
        let player = Player::new(100, 1000); // Changed from 50 to 1000 gold
        
        Ok(Self {
            world,
            player,
            current_map: MapType::Beginner,
            wave: 0,
            game_time: 0.0,
            wave_cooldown: 5.0, // 5 seconds between waves
            game_state: GameState::Playing,
            projectiles: Vec::new(), // Initialize projectiles vector
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        
        if let GameState::Playing = self.game_state {
            self.game_time += dt;
            
            // Wave logic
            if self.wave_cooldown > 0.0 {
                self.wave_cooldown -= dt;
            } else {
                self.spawn_wave();
                self.wave += 1;
                self.wave_cooldown = 20.0; // 20 seconds between waves
            }
            
            // Update towers (detect enemies, shoot)
            let mut targets = Vec::new();
            
            // Collect enemy positions for tower targeting
            let enemy_positions: Vec<(f32, f32)> = self.world.query::<&Enemy>()
                .iter()
                .filter(|(_, enemy)| enemy.is_active())
                .map(|(_, enemy)| enemy.get_position())
                .collect();
            
            // Update towers and collect targets with damage information
            for (_, tower) in self.world.query::<&mut Tower>().iter() {
                if let Some(target) = tower.update(dt, &enemy_positions) {
                    // Create a projectile when tower fires
                    let projectile = Projectile {
                        start: Vec2::new(tower.get_position().0, tower.get_position().1),
                        target: Vec2::new(target.0, target.1),
                        time_alive: 0.0,
                        max_lifetime: 0.5, // Half a second lifetime for projectile
                        color: match tower.get_tower_type() {
                            TowerType::Basic => [0.2, 0.6, 0.8, 1.0],
                            TowerType::Advanced => [0.8, 0.2, 0.8, 1.0],
                        },
                    };
                    self.projectiles.push(projectile);
                    
                    targets.push((target, tower.get_damage()));
                }
            }
            
            // Apply damage to enemies that were hit by tower shots
            for (target_pos, damage) in targets {
                self.damage_enemy_at_position(target_pos, damage);
            }
            
            // Update existing projectiles
            let mut i = 0;
            while i < self.projectiles.len() {
                self.projectiles[i].time_alive += dt;
                if self.projectiles[i].time_alive >= self.projectiles[i].max_lifetime {
                    self.projectiles.swap_remove(i);
                } else {
                    i += 1;
                }
            }
            
            // Update enemies (movement, take damage)
            let map = self.get_current_map();
            let mut enemies_to_remove = Vec::new();
            let mut player_health_deduction = 0;
            let mut rewards_earned = 0;
            let mut score_earned = 0;
            
            for (id, enemy) in self.world.query::<&mut Enemy>().iter() {
                if enemy.update(dt, &map) {
                    // Enemy reached the end of path
                    player_health_deduction += enemy.get_damage();
                    enemies_to_remove.push(id);
                } else if enemy.get_health() == 0 {
                    // Enemy was killed
                    rewards_earned += enemy.get_reward();
                    score_earned += enemy.get_reward() * 10;
                    enemies_to_remove.push(id);
                }
            }
            
            // Apply damage and rewards
            if player_health_deduction > 0 {
                self.player.take_damage(player_health_deduction);
                if self.player.is_dead() {
                    self.game_state = GameState::GameOver;
                }
            }
            
            if rewards_earned > 0 {
                self.player.add_gold(rewards_earned);
                self.player.add_score(score_earned);
            }
            
            // Check for victory condition
            if self.wave >= 20 && self.world.query::<&Enemy>().iter().count() == 0 {
                self.game_state = GameState::Victory;
            }
        }
        
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw the map
        self.get_current_map().draw(ctx, canvas)?;
        
        // Draw all towers
        for (_id, tower) in self.world.query::<&Tower>().iter() {
            tower.draw(ctx, canvas)?;
        }
        
        // Draw all enemies
        for (_id, enemy) in self.world.query::<&Enemy>().iter() {
            enemy.draw(ctx, canvas)?;
        }
        
        // Draw projectiles
        for projectile in &self.projectiles {
            // Calculate position along trajectory based on lifetime
            let progress = projectile.time_alive / projectile.max_lifetime;
            let position = projectile.start.lerp(projectile.target, progress);
            
            // Draw the projectile
            let projectile_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                position,
                5.0, // Radius
                0.1, // Tolerance
                projectile.color.into(),
            )?;
            
            canvas.draw(&projectile_mesh, graphics::DrawParam::default());
            
            // Draw trail (optional - for more visual effect)
            if progress > 0.1 {
                let trail_start = projectile.start.lerp(projectile.target, progress - 0.1);
                let trail = graphics::Mesh::new_line(
                    ctx,
                    &[trail_start, position],
                    2.0, // Line width
                    [projectile.color[0], projectile.color[1], projectile.color[2], 0.5].into(), // Semi-transparent
                )?;
                
                canvas.draw(&trail, graphics::DrawParam::default());
            }
        }
        
        Ok(())
    }

    // Fix the implementation by moving this function inside the impl block
    fn damage_enemy_at_position(&mut self, position: (f32, f32), damage: u32) {
        // Find enemies close to the target position (with some tolerance)
        const HIT_TOLERANCE: f32 = 15.0; // Pixels of tolerance for hit detection
        
        for (_, enemy) in self.world.query::<&mut Enemy>().iter() {
            let enemy_pos = enemy.get_position();
            let dx = enemy_pos.0 - position.0;
            let dy = enemy_pos.1 - position.1;
            let distance_squared = dx * dx + dy * dy;
            
            // If enemy is close enough to the target point, damage it
            if distance_squared <= HIT_TOLERANCE * HIT_TOLERANCE {
                if enemy.take_damage(damage) {
                    // Enemy was killed by this shot, score and rewards will be collected
                    // in the main update loop
                    break;
                }
            }
        }
    }

    // Add a helper method to get the tower type
    pub fn get_current_map(&self) -> Map {
        Map::new(self.current_map)
    }

    fn spawn_wave(&mut self) {
        let num_enemies = 5 + self.wave as usize;
        let enemy_type = if self.wave % 5 == 0 {
            EnemyType::Boss
        } else if self.wave % 3 == 0 {
            EnemyType::Fast
        } else {
            EnemyType::Normal
        };
        
        let map = self.get_current_map();
        let start_pos = map.get_path_start();
        
        for i in 0..num_enemies {
            let enemy = Enemy::new(
                enemy_type,
                start_pos.0, 
                start_pos.1,
                2.0 + (i as f32 * 0.5), // Delay for each enemy
            );
            self.world.spawn((enemy,));
        }
    }
    
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub fn get_wave(&self) -> u32 {
        self.wave
    }
    
    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }
    
    pub fn toggle_pause(&mut self) {
        self.game_state = match &self.game_state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            state => state.clone(), // Clone the state instead of moving it
        };
    }

    pub fn place_tower(&mut self, tower_type: TowerType, x: f32, y: f32) -> bool {
        let tower_cost = tower_type.cost();
        
        // Check if player has enough gold and position is valid
        if self.player.get_gold() >= tower_cost && 
           self.get_current_map().can_place_tower(x, y) {
            
            let tower = Tower::new(tower_type, x, y);
            self.world.spawn((tower,));
            self.player.spend_gold(tower_cost);
            return true;
        } else {
            return false;
        }
    }
}
