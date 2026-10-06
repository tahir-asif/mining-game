use crate::constants::*;
use crate::level::block::{Block, BlockType, MiningOutcome};
use crate::level::coordinate::MapCoords;
use crate::level::levels::BlockId;

use macroquad::prelude::*;

pub struct GameMap {
    map: Vec<Option<Block>>,
    width: u8,
    height: u8,
}

impl GameMap {
    pub fn new(width: u8, height: u8) -> Self {
        let size = height * width;
        GameMap {
            map: vec![None; usize::from(size)],
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
        clear_background(SKYBLUE);

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
            return false;
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
        let i = coords.x * self.height + coords.z;
        usize::from(i)
    }

    fn get_block(&mut self, coords: MapCoords) -> Option<&mut Block> {
        if self.is_out_of_bounds(coords) {
            return None;
        }

        let i = self.index_map(coords);
        self.map[i].as_mut()
    }

    fn _add_block(&mut self, coords: MapCoords, kind: BlockType) {
        if self.is_block(coords) {
            return;
        }
        if self.is_out_of_bounds(coords) {
            return;
        }

        let i = self.index_map(coords);
        self.map[i] = Some(Block::new(kind));
    }

    fn add_block_from_id(&mut self, coords: MapCoords, id: BlockId) {
        if self.is_block(coords) || self.is_out_of_bounds(coords) {
            return;
        }

        if let Some(block_kind) = self.block_from_id(id) {
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

    fn block_from_id(&self, id: BlockId) -> Option<BlockType> {
        match id {
            0 => None,
            1 => Some(BlockType::Wall1),
            2 => Some(BlockType::Rock1),
            3 => Some(BlockType::Rock2),
            4 => Some(BlockType::Rock3),
            5 => Some(BlockType::Ore1),
            6 => Some(BlockType::Crystal1),
            7 => Some(BlockType::Chest1),
            _ => None,
        }
    }
}
