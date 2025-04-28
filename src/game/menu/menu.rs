use crate::game::GameState; // Assuming this exists
use ggez::event::KeyCode;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::{Context, GameResult};

#[derive(Debug, Clone, PartialEq)]
pub enum MenuOption {
    Resume,
    GameSpeed(f32),
    Difficulty(DifficultyLevel),
    SoundVolume(f32),
    MusicVolume(f32),
    QuitToMainMenu,
    ExitGame,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
}

impl DifficultyLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "Easy",
            DifficultyLevel::Normal => "Normal",
            DifficultyLevel::Hard => "Hard",
        }
    }
    
    pub fn next(&self) -> Self {
        match self {
            DifficultyLevel::Easy => DifficultyLevel::Normal,
            DifficultyLevel::Normal => DifficultyLevel::Hard,
            DifficultyLevel::Hard => DifficultyLevel::Easy,
        }
    }
}

pub struct PauseMenu {
    options: Vec<MenuOption>,
    selected_index: usize,
    game_speed: f32,
    difficulty: DifficultyLevel,
    sound_volume: f32,
    music_volume: f32,
}

impl PauseMenu {
    pub fn new() -> Self {
        // Default values
        let game_speed = 1.0;
        let difficulty = DifficultyLevel::Normal;
        let sound_volume = 0.8;
        let music_volume = 0.7;
        
        let options = vec![
            MenuOption::Resume,
            MenuOption::GameSpeed(game_speed),
            MenuOption::Difficulty(difficulty),
            MenuOption::SoundVolume(sound_volume),
            MenuOption::MusicVolume(music_volume),
            MenuOption::QuitToMainMenu,
            MenuOption::ExitGame,
        ];
        
        Self {
            options,
            selected_index: 0,
            game_speed,
            difficulty,
            sound_volume,
            music_volume,
        }
    }
    
    pub fn handle_key_press(&mut self, key: KeyCode) -> Option<GameState> {
        match key {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                } else {
                    self.selected_index = self.options.len() - 1;
                }
                None
            }
            KeyCode::Down => {
                if self.selected_index < self.options.len() - 1 {
                    self.selected_index += 1;
                } else {
                    self.selected_index = 0;
                }
                None
            }
            KeyCode::Left => {
                self.adjust_option(-1);
                None
            }
            KeyCode::Right => {
                self.adjust_option(1);
                None
            }
            KeyCode::Return => {
                self.select_option()
            }
            KeyCode::Escape => {
                Some(GameState::Playing)
            }
            _ => None,
        }
    }
    
    fn adjust_option(&mut self, direction: i32) {
        match &self.options[self.selected_index] {
            MenuOption::GameSpeed(_) => {
                self.game_speed = (self.game_speed + 0.1 * direction as f32).max(0.5).min(2.0);
                self.options[self.selected_index] = MenuOption::GameSpeed(self.game_speed);
            }
            MenuOption::SoundVolume(_) => {
                self.sound_volume = (self.sound_volume + 0.05 * direction as f32).max(0.0).min(1.0);
                self.options[self.selected_index] = MenuOption::SoundVolume(self.sound_volume);
            }
            MenuOption::MusicVolume(_) => {
                self.music_volume = (self.music_volume + 0.05 * direction as f32).max(0.0).min(1.0);
                self.options[self.selected_index] = MenuOption::MusicVolume(self.music_volume);
            }
            MenuOption::Difficulty(_) => {
                self.difficulty = self.difficulty.next();
                self.options[self.selected_index] = MenuOption::Difficulty(self.difficulty);
            }
            _ => {}
        }
    }
    
    fn select_option(&self) -> Option<GameState> {
        match &self.options[self.selected_index] {
            MenuOption::Resume => Some(GameState::Playing),
            MenuOption::QuitToMainMenu => Some(GameState::MainMenu),
            MenuOption::ExitGame => Some(GameState::Exit),
            _ => None, // For options that are adjusted with left/right
        }
    }
    
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut y = 150.0;
        let line_height = 40.0;
        
        for (i, option) in self.options.iter().enumerate() {
            let option_text = match option {
                MenuOption::Resume => "Resume".to_string(),
                MenuOption::GameSpeed(speed) => format!("Game Speed: {:.1}x", speed),
                MenuOption::Difficulty(level) => format!("Difficulty: {}", level.as_str()),
                MenuOption::SoundVolume(vol) => format!("Sound Volume: {:.0}%", vol * 100.0),
                MenuOption::MusicVolume(vol) => format!("Music Volume: {:.0}%", vol * 100.0),
                MenuOption::QuitToMainMenu => "Quit to Main Menu".to_string(),
                MenuOption::ExitGame => "Exit Game".to_string(),
            };
            
            let text = Text::new(option_text);
            let color = if i == self.selected_index {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            
            graphics::draw(
                ctx,
                &text,
                DrawParam::default()
                    .dest([400.0, y])
                    .color(color)
                    .offset([0.5, 0.0]),
            )?;
            
            y += line_height;
        }
        
        Ok(())
    }
    
    // Getters for game settings
    pub fn game_speed(&self) -> f32 {
        self.game_speed
    }
    
    pub fn difficulty(&self) -> DifficultyLevel {
        self.difficulty
    }
    
    pub fn sound_volume(&self) -> f32 {
        self.sound_volume
    }
    
    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }
}
