use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 1.0;

pub struct GameMap {
    pub blocks: Vec<(i32, i32)>,
}

impl GameMap {
    pub fn grid(&mut self) {
        self.new_block(2, 4);
    }

    pub fn is_block(&mut self, x: i32, y: i32) -> bool {
        self.blocks.contains(&(x, y))
    }

    fn new_block(&mut self, x: i32, y: i32) {
        if self.is_block(x, y) {
            return;
        }
        self.blocks.push((x, y));
    }

    pub fn draw(&mut self) {
        clear_background(GRAY);

        for (x, y) in &self.blocks {
            draw_cube(
                vec3(GRID_SIZE * (*x as f32), 0.0, GRID_SIZE * (*y as f32)),
                vec3(GRID_SIZE * 0.8, GRID_SIZE * 0.8, GRID_SIZE * 0.8),
                None,
                GREEN,
            );
        }
    }
}
