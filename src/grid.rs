use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Block {
    x: usize,
    y: usize,
    health: usize,
}

#[derive(Default)]
pub struct GameMap {
    pub map: Vec<Vec<Option<Block>>>,
    pub width: usize,
    pub height: usize,
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
        self.new_block(3, 3, 5);
        self.new_block(1, 2, 4);
        self.new_block(5, 3, 3);
    }

    fn get_block(&mut self, x: usize, y: usize) -> Option<&mut Block> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.map[x][y].as_mut()
    }

    pub fn is_block(&mut self, x: usize, y: usize) -> bool {
        self.get_block(x, y).is_some()
    }

    fn new_block(&mut self, x: usize, y: usize, health: usize) {
        if self.is_block(x, y) {
            return;
        }
        if x < self.width || y < self.height {
            self.map[x][y] = Some(Block { x, y, health })
        }
    }

    fn remove_block(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.map[x][y] = None;
        }
    }

    pub fn mine_block(&mut self, x: usize, y: usize, mining_power: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        match self.get_block(x, y) {
            None => {}
            Some(block) => {
                block.health = block.health.saturating_sub(mining_power);
                if block.health == 0 {
                    self.remove_block(x, y);
                }
            }
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
