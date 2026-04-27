use macroquad::prelude::*;

use crate::grid::GRID_SIZE;

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

pub struct CameraSettings {
    pub pos: Point,
    pub up: Point,
    pub tar: Point,
}

impl CameraSettings {
    pub fn set(&mut self) {
        let p = vec3(
            GRID_SIZE * self.pos.x as f32,
            GRID_SIZE * self.pos.y as f32,
            GRID_SIZE * self.pos.z as f32,
        );
        let u = vec3(
            GRID_SIZE * self.up.x as f32,
            GRID_SIZE * self.up.y as f32,
            GRID_SIZE * self.up.z as f32,
        );
        let t = vec3(
            GRID_SIZE * self.tar.x as f32,
            GRID_SIZE * self.tar.y as f32,
            GRID_SIZE * self.tar.z as f32,
        );
        set_camera(&Camera3D {
            position: p,
            up: u,
            target: t,
            ..Default::default()
        });
    }
}
