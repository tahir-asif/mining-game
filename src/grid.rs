use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone)]
struct Block {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct GameMap {
    pub map: Vec<Vec<Option<Block>>>,
    width: usize,
    height: usize,
}
impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        GameMap {
            map: vec![vec![None; height]; width],
            width,
            height,
        }
    }

    pub fn generate_level(&mut self) {
        self.new_block(3, 3);
        self.new_block(1, 3);
        self.new_block(5, 3);
    }

    fn get_block(&mut self, x: usize, y: usize) -> Option<&Block> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.map[x][y].as_ref()
    }

    pub fn is_block(&mut self, x: usize, y: usize) -> bool {
        self.get_block(x, y).is_some()
    }

    fn new_block(&mut self, x: usize, y: usize) {
        if self.is_block(x, y) {
            return;
        }
        if x < self.width || y < self.height {
            self.map[x][y] = Some(Block { x, y })
        }
    }

    pub fn remove_block(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.map[x][y] = None;
        }
    }

    pub fn draw(&mut self) {
        clear_background(GRAY);

        for row in &self.map {
            for option_block in row {
                match option_block {
                    None => continue,
                    Some(block) => {
                        draw_cube(
                            vec3(
                                GRID_SIZE * (block.x as f32),
                                0.0,
                                GRID_SIZE * (block.y as f32),
                            ),
                            vec3(GRID_SIZE * 0.8, GRID_SIZE * 0.8, GRID_SIZE * 0.8),
                            None,
                            GREEN,
                        );
                    }
                }
            }
        }
    }
}
