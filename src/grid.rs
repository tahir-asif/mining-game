pub mod block;

use crate::constants::*;
use crate::grid::block::{Block, MiningDrop, MiningOutcome};

use macroquad::prelude::*;

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
        for i in 0..self.width + 1 {
            self.new_block(i, 0, Block::Wall);
            self.new_block(i, self.height - 1, Block::Wall);
        }
        for i in 0..self.height + 1 {
            self.new_block(0, i, Block::Wall);
            self.new_block(self.width - 1, i, Block::Wall);
        }
        self.new_block(3, 3, Block::Rock { health: 9 });
        self.new_block(2, 7, Block::Rock { health: 5 });
        self.new_block(5, 4, Block::Rock { health: 2 });
        self.new_block(
            6,
            6,
            Block::Ore {
                health: 3,
                mining_drop: MiningDrop::Gold,
            },
        );
        self.new_block(
            8,
            8,
            Block::Chest {
                mining_drop: MiningDrop::Item,
            },
        );
        self.new_block(
            2,
            2,
            Block::Crystal {
                mining_drop: MiningDrop::Energy(10),
            },
        );
    }

    pub fn is_block(&mut self, x: usize, y: usize) -> bool {
        self.get_block(x, y).is_some()
    }

    pub fn mine_block(&mut self, x: usize, y: usize, mining_power: usize) -> Option<MiningOutcome> {
        match self.get_block(x, y) {
            None => None,
            Some(block) => match block.mine(mining_power) {
                MiningOutcome::Damaged => Some(MiningOutcome::Damaged),
                MiningOutcome::Destroyed => {
                    self.remove_block(x, y);
                    Some(MiningOutcome::Destroyed)
                }
                MiningOutcome::Gained(mining_drop) => {
                    self.remove_block(x, y);
                    Some(MiningOutcome::Gained(mining_drop))
                }
                MiningOutcome::Unbreakable => Some(MiningOutcome::Unbreakable),
            },
        }
    }

    pub fn draw(&mut self) {
        clear_background(GRAY);

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(block) = &self.map[x][y] {
                    draw_cube(
                        vec3(GRID_SIZE * (x as f32), 0.0, GRID_SIZE * (y as f32)),
                        vec3(GRID_SIZE * 0.8, GRID_SIZE * 0.8, GRID_SIZE * 0.8),
                        None,
                        block.colour(),
                    );
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

    fn new_block(&mut self, x: usize, z: usize, kind: Block) {
        if self.is_block(x, z) {
            return;
        }
        if self.is_out_of_bounds(x, z) {
            return;
        }
        self.map[x][z] = Some(kind)
    }

    fn remove_block(&mut self, x: usize, y: usize) {
        if self.is_out_of_bounds(x, y) {
            return;
        }
        self.map[x][y] = None;
    }
}
