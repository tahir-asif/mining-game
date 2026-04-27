use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 10.0;

pub fn grid() {
    block(1, 1);
    block(2, 2);
    block(4, 2);
    block(3, 2);
    block(2, 4);
    block(2, 4);
    block(3, 3);
}

fn block(x: u32, y: u32) {
    draw_cube(
        vec3((x as f32) * GRID_SIZE, 0.0, (y as f32) * GRID_SIZE),
        vec3(GRID_SIZE * 0.8, GRID_SIZE * 0.8, GRID_SIZE * 0.8),
        None,
        GREEN,
    );
}
