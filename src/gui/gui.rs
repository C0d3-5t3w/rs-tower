use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::event::MouseButton;
use glam::Vec2;

use crate::game::Game;
use crate::game::tower::TowerType;
use crate::game::GameState;

pub struct Gui {
    selected_tower: Option<TowerType>,
    ui_state: UiState,
}

enum UiState {
    Main,
    TowerPlacement,
    GameOver,
    Victory,
}

impl Gui {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            selected_tower: None,
            ui_state: UiState::Main,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, game: &mut Game) -> GameResult {
        match game.get_game_state() {
            GameState::GameOver => self.ui_state = UiState::GameOver,
            GameState::Victory => self.ui_state = UiState::Victory,
            _ => {}
        }
        
        // Update to use context properties instead of deprecated functions
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let mouse_pos = ctx.mouse.position();
            
            // Check UI button clicks
            if mouse_pos.y >= 700.0 {  // Adjusted for larger window
                if mouse_pos.x < 200.0 {
                    // Basic tower button
                    self.selected_tower = Some(TowerType::Basic);
                    self.ui_state = UiState::TowerPlacement;
                } else if mouse_pos.x < 400.0 {
                    // Advanced tower button
                    self.selected_tower = Some(TowerType::Advanced);
                    self.ui_state = UiState::TowerPlacement;
                } else if mouse_pos.x < 600.0 {
                    // Upgrade tower button (implementation would go here)
                    // For now just cancel selection
                    self.selected_tower = None;
                    self.ui_state = UiState::Main;
                } else if mouse_pos.x < 800.0 {
                    // Sell tower button (implementation would go here)
                    self.selected_tower = None;
                    self.ui_state = UiState::Main;
                } else {
                    // Cancel/Pause button
                    game.toggle_pause();
                }
            } else if let UiState::TowerPlacement = self.ui_state {
                // Place tower
                if let Some(tower_type) = self.selected_tower {
                    if game.place_tower(tower_type, mouse_pos.x, mouse_pos.y) {
                        self.selected_tower = None;
                        self.ui_state = UiState::Main;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, game: &Game, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw UI background - adjust for larger window
        let ui_bg = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 700.0, 1024.0, 68.0),
            [0.2, 0.2, 0.2, 1.0].into(),
        )?;
        
        canvas.draw(&ui_bg, DrawParam::default());
        
        // Draw player stats
        let player = game.get_player();
        let stats_text = Text::new(format!(
            "Health: {} | Gold: {} | Score: {} | Wave: {}",
            player.get_health(),
            player.get_gold(),
            player.get_score(),
            game.get_wave()
        ));
        
        canvas.draw(
            &stats_text,
            DrawParam::default()
                .dest(Vec2::new(10.0, 10.0))
                .color([1.0f32, 1.0f32, 1.0f32, 1.0f32]),
        );
        
        // Draw buttons - adjust for larger window
        let buttons = [
            ("Basic Tower (50g)", [0.5f32, 0.5f32, 1.0f32, 1.0f32]),
            ("Advanced Tower (100g)", [0.8f32, 0.4f32, 0.8f32, 1.0f32]),
            ("Upgrade Tower (75g)", [0.2f32, 0.8f32, 0.2f32, 1.0f32]),
            ("Sell Tower", [0.8f32, 0.7f32, 0.2f32, 1.0f32]),
            ("Cancel", [0.8f32, 0.2f32, 0.2f32, 1.0f32]),
        ];
        
        for (i, (label, color)) in buttons.iter().enumerate() {
            let button_rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(i as f32 * 200.0, 710.0, 190.0, 40.0),
                (*color).into(),
            )?;
            
            canvas.draw(&button_rect, DrawParam::default());
            
            let button_text = Text::new(*label);
            canvas.draw(
                &button_text,
                DrawParam::default()
                    .dest(Vec2::new(i as f32 * 200.0 + 10.0, 720.0))
                    .color([1.0f32, 1.0f32, 1.0f32, 1.0f32]),
            );
        }
        
        // Highlight selected tower type
        if let UiState::TowerPlacement = self.ui_state {
            // Draw a box around the selected tower type
            let highlight_index = match self.selected_tower {
                Some(TowerType::Basic) => 0,
                Some(TowerType::Advanced) => 1,
                None => 4, // Cancel button
            };
            
            let highlight_rect = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(2.0),
                Rect::new(highlight_index as f32 * 200.0, 710.0, 190.0, 40.0),
                [1.0f32, 1.0f32, 0.0f32, 1.0f32].into(), // Added .into() to convert to Color
            )?;
            
            canvas.draw(&highlight_rect, DrawParam::default());
        }

        // Draw game state messages
        match self.ui_state {
            UiState::GameOver => {
                let game_over_text = Text::new("GAME OVER");
                canvas.draw(
                    &game_over_text,
                    DrawParam::default()
                        .dest(Vec2::new(350.0, 250.0))
                        .color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
                        .scale(Vec2::new(3.0, 3.0)),
                );
            },
            UiState::Victory => {
                let victory_text = Text::new("VICTORY!");
                canvas.draw(
                    &victory_text,
                    DrawParam::default()
                        .dest(Vec2::new(350.0, 250.0))
                        .color([0.0f32, 1.0f32, 0.0f32, 1.0f32])
                        .scale(Vec2::new(3.0, 3.0)),
                );
            },
            UiState::TowerPlacement => {
                // Draw placement indicator
                let mouse_pos = ctx.mouse.position();
                let tower_range_circle = Mesh::new_circle(
                    ctx,
                    DrawMode::stroke(2.0),
                    Vec2::new(mouse_pos.x, mouse_pos.y),
                    match self.selected_tower {
                        Some(TowerType::Basic) => 100.0,
                        Some(TowerType::Advanced) => 150.0,
                        None => 0.0,
                    },
                    0.1,
                    [1.0, 1.0, 1.0, 0.5].into(),
                )?;
                
                canvas.draw(&tower_range_circle, DrawParam::default());
            },
            _ => {}
        }
        
        Ok(())
    }
}
