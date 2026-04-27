use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 10.0;

pub fn grid() {
    draw_grid_ex(
        100,
        GRID_SIZE,
        RED,
        BLUE,
        vec3(GRID_SIZE / 2.0, 0.0, GRID_SIZE / 2.0),
        Quat::IDENTITY,
    );

    // blocks
    draw_cube(vec3(1.0, 0.0, 1.0), vec3(1.0, 1.0, 1.0), None, GREEN);
}
