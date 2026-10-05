use crate::constants::GRID_SIZE;
use crate::level::block::{Drop, DropKind};
use crate::level::coordinate::MapCoords;

use macroquad::prelude::*;

pub struct Player {
    coords: MapCoords,
    pub energy: u16,
    pub tech: u16,
    pub mining_power: u16,
    pub collected_gold: u16,
}

impl Player {
    pub const fn new(spawn: MapCoords, energy: u16, tech: u16, mining_power: u16) -> Self {
        Player {
            coords: spawn,
            energy,
            tech,
            mining_power,
            collected_gold: 0,
        }
    }

    pub fn get_coords(&self) -> MapCoords {
        self.coords
    }

    pub fn move_to(&mut self, to: MapCoords) {
        self.coords = to;
    }

    pub fn spend_energy(&mut self) {
        self.energy -= 1;
    }

    pub fn collect_drops(&mut self, drops: &'static [Drop]) {
        for drop in drops {
            match drop.kind {
                DropKind::Energy => {
                    self.energy += drop.amount;
                }
                DropKind::Gold => {
                    self.collected_gold += drop.amount;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        let centre = vec3(
            GRID_SIZE * self.coords.x as f32,
            0.0,
            GRID_SIZE * self.coords.z as f32,
        );
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }
}
