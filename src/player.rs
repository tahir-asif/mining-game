use macroquad::prelude::*;

pub struct Player {
    pub x_pos: f32,
    pub y_pos: f32,
}

impl Player {
    pub fn handle_input(&mut self) {
        let grid_size: f32 = 32.0;
        if is_key_pressed(KeyCode::Right) {
            self.x_pos += grid_size;
        }
        if is_key_pressed(KeyCode::Left) {
            self.x_pos -= grid_size;
        }
        if is_key_pressed(KeyCode::Down) {
            self.y_pos += grid_size;
        }
        if is_key_pressed(KeyCode::Up) {
            self.y_pos -= grid_size;
        }
    }

    pub fn draw(&mut self) {
        draw_circle(self.x_pos, self.y_pos, 16.0, YELLOW);
    }
}
