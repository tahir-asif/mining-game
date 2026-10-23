use crate::constants::*;
use crate::level::block::{Block, MiningOutcome};
use crate::level::coordinate::MapCoords;
use crate::level::levels::BlockId;
use shared::block::{BlockType, blocktype_from_id};

use macroquad::prelude::*;

pub enum Reveal {
    Player,
    Spawn,
    Torch,
}

pub struct GameMap {
    map: Vec<Option<Block>>,
    fog_of_war_map: Vec<bool>,
    width: u8,
    height: u8,
}

impl GameMap {
    pub fn new(width: u8, height: u8) -> Self {
        let size = usize::from(width + 2) * usize::from(height + 2);
        GameMap {
            map: vec![None; size],
            fog_of_war_map: vec![true; size],
            width,
            height,
        }
    }

    pub fn generate_level(&mut self, block_ids: &[BlockId], spawn: MapCoords) {
        self.reveal_map(spawn, Reveal::Spawn);

        let full_width = self.width + 2;
        let full_height = self.height + 2;

        for x in 0..full_width {
            self.set_wall(MapCoords::new(x, 0));
            self.set_wall(MapCoords::new(x, full_height - 1));
        }
        for z in 0..full_height {
            self.set_wall(MapCoords::new(0, z));
            self.set_wall(MapCoords::new(full_width - 1, z));
        }

        for (i, id) in block_ids.iter().enumerate() {
            let x = (i % usize::from(self.width)) as u8 + 1;
            let z = (i / usize::from(self.width)) as u8 + 1;
            self.add_block_from_id(MapCoords::new(x, z), *id);
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        let full_width = f32::from(self.width) + 2.0;
        let full_height = f32::from(self.height) + 2.0;
        let centre = vec3(
            (full_width - 1.0) * 0.5 * GRID_SIZE,
            -0.5 * GRID_SIZE,
            (full_height - 1.0) * 0.5 * GRID_SIZE,
        );
        let half = vec2(full_width * 0.5 * GRID_SIZE, full_height * 0.5 * GRID_SIZE);
        draw_plane(centre, half, None, Color::new(0.12, 0.12, 0.14, 1.0));

        for x in 0..(self.width + 2) {
            for z in 0..(self.height + 2) {
                let coords = MapCoords::new(x, z);
                let i = self.index_map(coords);
                if self.fog_of_war_map[i] {
                    draw_block(coords, Color::from_hex(0x000000));
                } else if let Some(block) = &self.map[i] {
                    draw_block(coords, block.colour());
                }
            }
        }
    }

    pub fn reveal_map(&mut self, coords: MapCoords, what: Reveal) {
        let radius = match what {
            Reveal::Player => PLAYER_EXPLORE_RADIUS,
            Reveal::Torch => TORCH_EXPLORE_RADIUS,
            Reveal::Spawn => SPAWN_EXPLORE_RADIUS,
        };

        let min_x = coords.x.saturating_sub(radius);
        let max_x = coords.x.saturating_add(radius + 1);
        let min_z = coords.z.saturating_sub(radius);
        let max_z = coords.z.saturating_add(radius + 1);

        for x in min_x..max_x {
            for z in min_z..max_z {
                let i = self.index_map(MapCoords::new(x, z));
                self.fog_of_war_map[i] = false;
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
        self.map[i].is_some()
    }

    fn is_out_of_bounds(&self, coords: MapCoords) -> bool {
        coords.x >= self.width + 2 || coords.z >= self.height + 2
    }

    fn index_map(&self, coords: MapCoords) -> usize {
        usize::from(coords.x) * usize::from(self.height + 2) + usize::from(coords.z)
    }

    fn get_block(&mut self, coords: MapCoords) -> Option<&mut Block> {
        if self.is_out_of_bounds(coords) {
            return None;
        }

        let i = self.index_map(coords);
        self.map[i].as_mut()
    }

    fn set_wall(&mut self, coords: MapCoords) {
        let i = self.index_map(coords);
        self.map[i] = Some(Block::new(BlockType::Wall1));
    }

    fn add_block_from_id(&mut self, coords: MapCoords, id: BlockId) {
        if self.is_block(coords) || self.is_out_of_bounds(coords) {
            return;
        }

        if let Some(block_kind) = blocktype_from_id(id) {
            let i = self.index_map(coords);
            self.map[i] = Some(Block::new(block_kind));

            if block_kind == BlockType::Torch {
                self.reveal_map(coords, Reveal::Torch);
            }
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

fn draw_block(coords: MapCoords, colour: Color) {
    draw_cube(
        vec3(
            GRID_SIZE * coords.x as f32,
            0.0,
            GRID_SIZE * coords.z as f32,
        ),
        vec3(GRID_SIZE * 1.0, GRID_SIZE * 1.0, GRID_SIZE * 1.0),
        None,
        colour,
    );
}
