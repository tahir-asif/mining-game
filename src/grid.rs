use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone)]
struct Block {
    x: usize,
    y: usize,
    health: usize,
}
impl Block {
    fn new(x: usize, y: usize, health: usize) -> Self {
        Block { x, y, health }
    }

    fn mine(&mut self, mining_power: usize) {
        self.health = self.health.saturating_sub(mining_power);
    }

    fn colour(&self) -> Color {
        match self.health {
            1 => RED,
            2 => ORANGE,
            3 => YELLOW,
            4 => GREEN,
            5 => BLUE,
            6 => PINK,
            7 => PURPLE,
            _ => BLACK,
        }
    }
}

#[derive(Default)]
pub struct GameMap {
    map: Vec<Vec<Option<Block>>>,
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

    pub fn is_block(&mut self, x: usize, y: usize) -> bool {
        self.get_block(x, y).is_some()
    }

    pub fn mine_block(&mut self, x: usize, y: usize, mining_power: usize) {
        if self.is_out_of_bounds(x, y) {
            return;
        }
        match self.get_block(x, y) {
            None => {}
            Some(block) => {
                block.mine(mining_power);
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
                            block.colour(),
                        );
                    }
                }
            }
        }
    }

    // helper functions
    fn is_out_of_bounds(&self, x: usize, z: usize) -> bool {
        x >= self.width || z >= self.height
    }

    fn get_block(&mut self, x: usize, y: usize) -> Option<&mut Block> {
        if self.is_out_of_bounds(x, y) {
            return None;
        }
        self.map[x][y].as_mut()
    }

    fn new_block(&mut self, x: usize, z: usize, health: usize) {
        if self.is_block(x, z) {
            return;
        }
        if self.is_out_of_bounds(x, z) {
            return;
        }
        self.map[x][z] = Some(Block::new(x, z, health))
    }

    fn remove_block(&mut self, x: usize, y: usize) {
        if self.is_out_of_bounds(x, y) {
            return;
        }
        self.map[x][y] = None;
    }
}
