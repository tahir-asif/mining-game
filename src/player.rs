use macroquad::prelude::*;

use crate::debug::CameraSettings;

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
        let grid_size: f32 = 1.0;
        if is_key_pressed(KeyCode::Right) {
            self.x_pos += grid_size;
            cam.tar.x += grid_size;
            cam.pos.x += grid_size;
        }
        if is_key_pressed(KeyCode::Left) {
            self.x_pos -= grid_size;
            cam.tar.x -= grid_size;
            cam.pos.x -= grid_size;
        }
        if is_key_pressed(KeyCode::Down) {
            self.z_pos += grid_size;
            cam.tar.z += grid_size;
            cam.pos.z += grid_size;
        }
        if is_key_pressed(KeyCode::Up) {
            self.z_pos -= grid_size;
            cam.tar.z -= grid_size;
            cam.pos.z -= grid_size;
        }
    }

    pub fn draw(&mut self) {
        let centre = vec3(self.x_pos, 0.0, self.z_pos);
        draw_sphere(centre, 0.3, None, YELLOW);
    }
}
