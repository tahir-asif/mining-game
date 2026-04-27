use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 1.0;

pub fn grid() {
    draw_grid_ex(
        100,
        GRID_SIZE,
        RED,
        BLUE,
        vec3(0.0, 0.0, 0.0),
        Quat::IDENTITY,
    );

    // blocks
    draw_cube(vec3(1.0, 0.0, 1.0), vec3(1.0, 1.0, 1.0), None, GREEN);
}
