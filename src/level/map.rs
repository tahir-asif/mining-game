use crate::constants::*;
use crate::level::block::{Block, MiningDrop, MiningOutcome};
use crate::level::coordinate::MapCoords;

use macroquad::prelude::*;

pub struct GameMap {
    map: Vec<Option<Block>>,
    width: usize,
    height: usize,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        GameMap {
            map: vec![None; height * width],
            width,
            height,
        }
    }

    pub fn generate_level(&mut self) {
        self.temp();
    }

    pub fn draw(&self) {
        clear_background(GRAY);

        for x in 0..self.width {
            for z in 0..self.height {
                let i = self.index_map(MapCoords::new(x, z));
                if let Some(block) = &self.map[i] {
                    draw_cube(
                        vec3(GRID_SIZE * (x as f32), 0.0, GRID_SIZE * (z as f32)),
                        vec3(GRID_SIZE * 0.8, GRID_SIZE * 0.8, GRID_SIZE * 0.8),
                        None,
                        block.colour(),
                    );
                }
            }
        }
    }

    pub fn mine_block(&mut self, coords: MapCoords, mining_power: usize) -> Option<MiningOutcome> {
        match self.get_block(coords) {
            None => None,
            Some(block) => match block.mine(mining_power) {
                MiningOutcome::Damaged => Some(MiningOutcome::Damaged),
                MiningOutcome::Destroyed => {
                    self.remove_block(coords);
                    Some(MiningOutcome::Destroyed)
                }
                MiningOutcome::Gained(mining_drop) => {
                    self.remove_block(coords);
                    Some(MiningOutcome::Gained(mining_drop))
                }
                MiningOutcome::Unbreakable => Some(MiningOutcome::Unbreakable),
            },
        }
    }

    pub fn is_block(&self, coords: MapCoords) -> bool {
        if self.is_out_of_bounds(coords) {
            return false;
        }

        let i = self.index_map(coords);
        if self.map[i].is_some() {
            return true;
        }

        false
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    // helper functions
    fn is_out_of_bounds(&self, coords: MapCoords) -> bool {
        coords.x >= self.width || coords.z >= self.height
    }

    fn index_map(&self, coords: MapCoords) -> usize {
        coords.x * self.height + coords.z
    }

    fn get_block(&mut self, coords: MapCoords) -> Option<&mut Block> {
        if self.is_out_of_bounds(coords) {
            return None;
        }

        let i = self.index_map(coords);
        self.map[i].as_mut()
    }

    fn add_block(&mut self, coords: MapCoords, kind: Block) {
        if self.is_block(coords) {
            return;
        }
        if self.is_out_of_bounds(coords) {
            return;
        }

        let i = self.index_map(coords);
        self.map[i] = Some(kind);
    }

    fn remove_block(&mut self, coords: MapCoords) {
        if self.is_out_of_bounds(coords) {
            return;
        }

        let i = self.index_map(coords);
        self.map[i] = None;
    }

    fn temp(&mut self) {
        for i in 0..self.width + 1 {
            self.add_block(MapCoords::new(i, 0), Block::Wall);
            self.add_block(MapCoords::new(i, self.height - 1), Block::Wall);
        }
        for i in 0..self.height + 1 {
            self.add_block(MapCoords::new(0, i), Block::Wall);
            self.add_block(MapCoords::new(self.width - 1, i), Block::Wall);
        }
        self.add_block(MapCoords::new(3, 3), Block::Rock { health: 9 });
        self.add_block(MapCoords::new(2, 7), Block::Rock { health: 5 });
        self.add_block(MapCoords::new(5, 4), Block::Rock { health: 2 });
        self.add_block(
            MapCoords::new(6, 6),
            Block::Ore {
                health: 3,
                mining_drop: MiningDrop::Gold(5),
            },
        );
        self.add_block(
            MapCoords::new(8, 8),
            Block::Chest {
                mining_drop: MiningDrop::Item,
            },
        );
        self.add_block(
            MapCoords::new(2, 2),
            Block::Crystal {
                mining_drop: MiningDrop::Energy(10),
            },
        );
    }
}
