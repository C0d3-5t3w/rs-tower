use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::conf::{WindowMode, WindowSetup};
use crate::game::Game;
use crate::gui::Gui;

mod game;
mod gui;

fn main() -> GameResult {
    // Create a game context with a larger, more detailed window
    let (mut ctx, event_loop) = ContextBuilder::new("rs-tower-defense", "rs-tower")
        .window_setup(WindowSetup::default()
            .title("Rust Tower Defense")
            .vsync(true))
        .window_mode(WindowMode::default()
            .dimensions(1024.0, 768.0)
            .resizable(false))
        .build()?;

    // Create game state
    let game = Game::new(&mut ctx)?;
    let gui = Gui::new(&mut ctx)?;
    
    // Create main game state
    let state = MainState { game, gui };
    
    // Run the game
    event::run(ctx, event_loop, state)
}

struct MainState {
    game: Game,
    gui: Gui,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update game logic
        self.game.update(ctx)?;
        
        // Update GUI
        self.gui.update(ctx, &mut self.game)?;
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create canvas with background color
        let mut canvas = graphics::Canvas::from_frame(ctx, Some([0.1, 0.2, 0.3, 1.0].into()));
        
        // Draw game elements
        self.game.draw(ctx, &mut canvas)?;
        
        // Draw GUI overlay
        self.gui.draw(ctx, &self.game, &mut canvas)?;
        
        // Finish drawing
        canvas.finish(ctx)?;
        Ok(())
    }
}
