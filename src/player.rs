use crate::camera::CameraSettings;
use crate::constants::*;
use crate::grid::GameMap;

use macroquad::prelude::*;

pub struct Player {
    // no y_pos because it should not move along y-axis; y is 3d up/down
    pub x_pos: usize,
    pub z_pos: usize,
}

impl Player {
    pub const fn new(x_pos: usize, z_pos: usize) -> Player {
        Player { x_pos, z_pos }
    }

    pub fn handle_input(&mut self, cam: &mut CameraSettings, game_map: &mut GameMap) {
        self.handle_movement(cam, game_map);
    }

    pub fn draw(&mut self) {
        let centre = vec3(
            GRID_SIZE * self.x_pos as f32,
            0.0,
            GRID_SIZE * self.z_pos as f32,
        );
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }

    fn handle_movement(&mut self, cam: &mut CameraSettings, game_map: &mut GameMap) {
        let (dx, dz): (isize, isize) = match get_last_key_pressed() {
            None => return,
            Some(KeyCode::Up) => (0, 1),
            Some(KeyCode::W) => (0, 1),
            Some(KeyCode::Down) => (0, -1),
            Some(KeyCode::S) => (0, -1),
            Some(KeyCode::Right) => (-1, 0),
            Some(KeyCode::D) => (-1, 0),
            Some(KeyCode::Left) => (1, 0),
            Some(KeyCode::A) => (1, 0),
            _ => return,
        };

        // mine if holding shift while moving
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            game_map.remove_block(
                self.x_pos.saturating_add_signed(dx),
                self.z_pos.saturating_add_signed(dz),
            );
        }

        // don't move if block is in the way
        if game_map.is_block(
            self.x_pos.saturating_add_signed(dx),
            self.z_pos.saturating_add_signed(dz),
        ) {
            return;
        }

        // move player in x direction
        let move_x = self.x_pos.saturating_add_signed(dx);
        if move_x <= game_map.width {
            self.x_pos = move_x;
            cam.pos.x = move_x as isize - CAM_DISTANCE;
            cam.tar.x = move_x as isize;
        }

        // move player in z direction
        let move_z = self.z_pos.saturating_add_signed(dz);
        if move_z <= game_map.width {
            self.z_pos = move_z;
            cam.pos.z = move_z as isize - CAM_DISTANCE;
            cam.tar.z = move_z as isize;
        }
    }
}
