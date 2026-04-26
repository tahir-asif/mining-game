use macroquad::prelude::*;

pub struct Player {
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    centre: Vec3,
}

impl Player {
    pub fn new(x_pos: f32, y_pos: f32, z_pos: f32) -> Player {
        Player {
            x_pos,
            y_pos,
            z_pos,
            centre: Vec3 {
                x: x_pos,
                y: y_pos,
                z: z_pos,
            },
        }
    }

    pub fn handle_input(&mut self) {
        let grid_size: f32 = 100.0;
        if is_key_pressed(KeyCode::Right) {
            self.x_pos += grid_size;
        }
        if is_key_pressed(KeyCode::Left) {
            self.x_pos -= grid_size;
        }
        if is_key_pressed(KeyCode::Down) {
            self.z_pos += grid_size;
        }
        if is_key_pressed(KeyCode::Up) {
            self.z_pos -= grid_size;
        }
    }

    pub fn draw(&mut self) {
        draw_sphere(self.centre, 1.0, None, YELLOW);
    }
}
