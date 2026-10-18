use crate::constants::*;
use crate::level::block::{Block, MiningOutcome};
use crate::level::coordinate::MapCoords;
use crate::level::levels::BlockId;
use shared::block::blocktype_from_id;

use macroquad::prelude::*;

pub struct GameMap {
    map: Vec<Option<Block>>,
    width: u8,
    height: u8,
}

impl GameMap {
    pub fn new(width: u8, height: u8) -> Self {
        let size = usize::from(width) * usize::from(height);
        GameMap {
            map: vec![None; size],
            width,
            height,
        }
    }

    pub fn generate_level(&mut self, block_ids: &[BlockId]) {
        for (i, id) in block_ids.iter().enumerate() {
            let x = i % usize::from(self.width);
            let z = i / usize::from(self.width);
            let x = u8::try_from(x).unwrap_or_else(|err| {
                panic!("Block {i} {id} coordinate {x} too big: {err}");
            });
            let z = u8::try_from(z).unwrap_or_else(|err| {
                panic!("Block {i} {id} coordinate {z} too big: {err}");
            });
            self.add_block_from_id(MapCoords::new(x, z), *id);
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        let width = f32::from(self.width);
        let height = f32::from(self.height);
        let centre = vec3(
            (width - 1.0) * 0.5 * GRID_SIZE,
            -0.5 * GRID_SIZE,
            (height - 1.0) * 0.5 * GRID_SIZE,
        );
        let half = vec2(width * 0.5 * GRID_SIZE, height * 0.5 * GRID_SIZE);
        draw_plane(centre, half, None, Color::new(0.12, 0.12, 0.14, 1.0));

        for x in 0..self.width {
            for z in 0..self.height {
                let i = self.index_map(MapCoords::new(x, z));
                if let Some(block) = &self.map[i] {
                    draw_cube(
                        vec3(GRID_SIZE * x as f32, 0.0, GRID_SIZE * z as f32),
                        vec3(GRID_SIZE * 0.9, GRID_SIZE * 0.9, GRID_SIZE * 0.9),
                        None,
                        block.colour(),
                    );
                }
            }
        }
    }
    pub fn mine_block(&mut self, coords: MapCoords, mining_power: u16) -> Option<MiningOutcome> {
        let block = self.get_block(coords)?;
        match block.mine(mining_power) {
            MiningOutcome::Unbreakable => None,
            MiningOutcome::Damaged => Some(MiningOutcome::Damaged),
            MiningOutcome::Destroyed => {
                self.remove_block(coords);
                Some(MiningOutcome::Destroyed)
            }
            MiningOutcome::Gained(drop) => {
                self.remove_block(coords);
                Some(MiningOutcome::Gained(drop))
            }
        }
    }

    pub fn is_block(&self, coords: MapCoords) -> bool {
        if self.is_out_of_bounds(coords) {
            return true;
        }

        let i = self.index_map(coords);
        if self.map[i].is_some() {
            return true;
        }

        false
    }

    pub fn get_width(&self) -> usize {
        usize::from(self.width)
    }

    pub fn get_height(&self) -> usize {
        usize::from(self.height)
    }

    // helper functions
    fn is_out_of_bounds(&self, coords: MapCoords) -> bool {
        coords.x >= self.width || coords.z >= self.height
    }

    fn index_map(&self, coords: MapCoords) -> usize {
        usize::from(coords.x) * usize::from(self.height) + usize::from(coords.z)
    }

    fn get_block(&mut self, coords: MapCoords) -> Option<&mut Block> {
        if self.is_out_of_bounds(coords) {
            return None;
        }

        let i = self.index_map(coords);
        self.map[i].as_mut()
    }

    fn add_block_from_id(&mut self, coords: MapCoords, id: BlockId) {
        if self.is_block(coords) || self.is_out_of_bounds(coords) {
            return;
        }

        if let Some(block_kind) = blocktype_from_id(id) {
            let i = self.index_map(coords);
            self.map[i] = Some(Block::new(block_kind));
        }
    }

    fn remove_block(&mut self, coords: MapCoords) {
        if self.is_out_of_bounds(coords) {
            return;
        }

        let i = self.index_map(coords);
        self.map[i] = None;
    }
}
