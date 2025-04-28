mod map1;
mod map2;

use ggez::GameResult;
use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect};
use glam::Vec2;

pub use map1::Map1;
pub use map2::Map2;

#[derive(Debug, Clone, Copy)]
pub enum MapType {
    Beginner,
    Advanced,
}

pub struct Map {
    map_type: MapType,
    path: Vec<(f32, f32)>,
    buildable_areas: Vec<(f32, f32, f32, f32)>, // x, y, width, height
}

impl Map {
    pub fn new(map_type: MapType) -> Self {
        match map_type {
            MapType::Beginner => Map1::create(),
            MapType::Advanced => Map2::create(),
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        // Apply different background color based on map type
        let bg_color = match self.map_type {
            MapType::Beginner => [0.1, 0.4, 0.2, 1.0],
            MapType::Advanced => [0.1, 0.2, 0.4, 1.0],
        };
        
        let bg_rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, 1024.0, 700.0), // Adjusted for larger window
            bg_color.into(),
        )?;
        
        canvas.draw(&bg_rect, DrawParam::default());
        
        // Draw path
        for i in 0..self.path.len() - 1 {
            let start = self.path[i];
            let end = self.path[i + 1];
            
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(start.0, start.1),
                    Vec2::new(end.0, end.1),
                ],
                20.0, // Path width
                [0.8, 0.7, 0.5, 1.0].into(),
            )?;
            
            canvas.draw(&line, DrawParam::default());
        }
        
        // Draw buildable areas with light highlighting
        for area in &self.buildable_areas {
            let area_rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(area.0, area.1, area.2, area.3),
                [0.2, 0.5, 0.3, 0.3].into(),
            )?;
            
            canvas.draw(&area_rect, DrawParam::default());
        }
        
        Ok(())
    }

    pub fn can_place_tower(&self, x: f32, y: f32) -> bool {
        // Check if in buildable area
        for area in &self.buildable_areas {
            if x >= area.0 && x <= area.0 + area.2 &&
               y >= area.1 && y <= area.1 + area.3 {
                return true;
            }
        }
        
        false
    }
    
    pub fn get_path(&self) -> &[(f32, f32)] {
        &self.path
    }
    
    pub fn get_path_start(&self) -> (f32, f32) {
        self.path[0]
    }
    
    // pub fn get_path_end(&self) -> (f32, f32) {
    //     // Return the last point in the path
    //     *self.path.last().unwrap_or(&(0.0, 0.0))
    // }

    // pub fn get_map_type(&self) -> MapType {
    //     self.map_type
    // }
}
