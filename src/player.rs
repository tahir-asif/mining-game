use macroquad::prelude::*;

use crate::{
    debug::CameraSettings,
    grid::{GRID_SIZE, GameMap},
};

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
            Some(KeyCode::Down) => (0, -1),
            Some(KeyCode::Right) => (-1, 0),
            Some(KeyCode::Left) => (1, 0),
            _ => return,
        };
        if game_map.is_block(self.x_pos + dz, self.z_pos + dz) {
            println!("is block at {0}, {1}", self.x_pos + dx, self.z_pos + dz);
            return;
        }
        self.x_pos += dx;
        self.z_pos += dz;
        cam.tar.x += GRID_SIZE * dx as f32;
        cam.tar.z += GRID_SIZE * dz as f32;
        cam.pos.x += GRID_SIZE * dx as f32;
        cam.pos.z += GRID_SIZE * dz as f32;
    }

    pub fn draw(&mut self) {
        let centre = vec3(
            GRID_SIZE * self.x_pos as f32,
            GRID_SIZE,
            GRID_SIZE * self.z_pos as f32,
        );
        draw_sphere(centre, GRID_SIZE / 3.0, None, YELLOW);
    }
}
