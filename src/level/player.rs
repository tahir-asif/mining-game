use crate::constants::GRID_SIZE;
use crate::level::block::{MiningDrop, MiningOutcome};
use crate::level::camera::Camera;
use crate::level::coordinate::MapCoords;
use crate::level::map::GameMap;

use macroquad::prelude::*;

pub enum Action {
    Left,
    Right,
    Up,
    Down,
}

pub struct Player {
    coords: MapCoords,
    pub energy: usize,
    pub gold: u8,
}

impl Player {
    pub const fn new() -> Self {
        Player {
            coords: MapCoords { x: 2, z: 2 },
            energy: 100,
            gold: 20,
        }
    }

    pub fn get_coords(&self) -> MapCoords {
        self.coords
    }

    pub fn handle_input(
        &mut self,
        game_map: &mut GameMap,
        camera: &mut Camera,
        action: Action,
        do_mine: bool,
    ) {
        let (dx, dz): (isize, isize) = match action {
            Action::Left => (1, 0),
            Action::Right => (-1, 0),
            Action::Up => (0, 1),
            Action::Down => (0, -1),
        };
        let (dx, dz) = self.coords.add((dx, dz));
        let move_to = MapCoords::new(dx, dz);

        if do_mine {
            let mine_was_successful = game_map.mine_block(move_to, 1);
            match mine_was_successful {
                Some(MiningOutcome::Damaged) => self.energy = self.energy.saturating_sub(1),
                Some(MiningOutcome::Destroyed) => self.energy = self.energy.saturating_sub(1),
                Some(MiningOutcome::Gained(MiningDrop::Gold(amount))) => {
                    self.energy = self.energy.saturating_sub(1);
                    self.gold += amount;
                }
                Some(MiningOutcome::Gained(MiningDrop::Energy(amount))) => {
                    self.energy += amount as usize
                }
                Some(MiningOutcome::Gained(_)) => {}
                Some(MiningOutcome::Unbreakable) => {}
                None => {}
            }
        }

        if game_map.is_block(move_to) {
            return;
        }
        if move_to.x <= game_map.get_width() {
            self.coords.x = move_to.x;
        }
        if move_to.z <= game_map.get_height() {
            self.coords.z = move_to.z;
        }
        camera.point(move_to);
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
