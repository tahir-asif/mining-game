use macroquad::prelude::*;

use crate::{debug::CameraSettings, grid::GRID_SIZE};

pub struct Player {
    // no y_pos because it should not move along y-axis
    pub x_pos: f32,
    pub z_pos: f32,
}

impl Player {
    pub fn new(x_pos: f32, z_pos: f32) -> Player {
        Player { x_pos, z_pos }
    }

    pub fn handle_input(&mut self, cam: &mut CameraSettings) {
        if is_key_pressed(KeyCode::Right) {
            self.x_pos -= GRID_SIZE;
            cam.tar.x -= GRID_SIZE;
            cam.pos.x -= GRID_SIZE;
        }
        if is_key_pressed(KeyCode::Left) {
            self.x_pos += GRID_SIZE;
            cam.tar.x += GRID_SIZE;
            cam.pos.x += GRID_SIZE;
        }
        if is_key_pressed(KeyCode::Down) {
            self.z_pos -= GRID_SIZE;
            cam.tar.z -= GRID_SIZE;
            cam.pos.z -= GRID_SIZE;
        }
        if is_key_pressed(KeyCode::Up) {
            self.z_pos += GRID_SIZE;
            cam.tar.z += GRID_SIZE;
            cam.pos.z += GRID_SIZE;
        }
    }

    pub fn draw(&mut self) {
        let centre = vec3(self.x_pos, 0.0, self.z_pos);
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }
}
