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
        let (dx, dz) = match get_last_key_pressed() {
            None => return,
            Some(KeyCode::Up) => (0.0, GRID_SIZE),
            Some(KeyCode::Down) => (0.0, -GRID_SIZE),
            Some(KeyCode::Right) => (GRID_SIZE, 0.0),
            Some(KeyCode::Left) => (-GRID_SIZE, 0.0),
            _ => return,
        };
        self.x_pos += dx;
        self.z_pos += dz;
        cam.tar.x += dx;
        cam.tar.z += dz;
        cam.pos.x += dx;
        cam.pos.z += dz;
    }

    pub fn draw(&mut self) {
        let centre = vec3(self.x_pos, 0.0, self.z_pos);
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }
}
