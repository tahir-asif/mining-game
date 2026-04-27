use crate::camera::CameraSettings;
use crate::grid::{GRID_SIZE, GameMap};

use macroquad::prelude::*;

pub struct Player {
    // no y_pos because it should not move along y-axis
    pub x_pos: i32,
    pub z_pos: i32,
}

impl Player {
    pub fn new(x_pos: i32, z_pos: i32) -> Player {
        Player { x_pos, z_pos }
    }

    pub fn handle_input(&mut self, cam: &mut CameraSettings, game_map: &mut GameMap) {
        let (dx, dz) = match get_last_key_pressed() {
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
        if game_map.is_block(self.x_pos + dx, self.z_pos + dz) {
            return;
        }
        self.x_pos += dx;
        self.z_pos += dz;
        cam.tar.x += dx;
        cam.tar.z += dz;
        cam.pos.x += dx;
        cam.pos.z += dz;
    }

    pub fn draw(&mut self) {
        let centre = vec3(
            GRID_SIZE * self.x_pos as f32,
            0.0,
            GRID_SIZE * self.z_pos as f32,
        );
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }
}
