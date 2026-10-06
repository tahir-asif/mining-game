use crate::constants::{CAM_DISTANCE, GRID_SIZE};
use crate::level::coordinate::{CameraCoords, MapCoords};
use crate::level::player::Player;

use macroquad::prelude::*;

pub struct Camera {
    pub pos: CameraCoords,
    pub up: CameraCoords,
    pub tar: CameraCoords,
}

impl Camera {
    pub fn new(player: &mut Player) -> Self {
        let mut camera = Camera {
            pos: CameraCoords::new(0, 0, 0),
            up: CameraCoords::new(0, 0, 0),
            tar: CameraCoords::new(0, 0, 0),
        };

        camera.reset(player);
        camera
    }

    pub fn reset(&mut self, player: &mut Player) {
        self.pos = CameraCoords::new(0, CAM_DISTANCE, 0);
        self.up = CameraCoords::new(0, 1, 0);
        self.tar = CameraCoords::new(0, 0, 0);
        self.point(player.get_coords());
    }

    pub fn point(&mut self, coords: MapCoords) {
        self.pos.x = (-CAM_DISTANCE).saturating_add_unsigned(usize::from(coords.x));
        self.pos.z = (-CAM_DISTANCE).saturating_add_unsigned(usize::from(coords.z));
        self.tar.x = isize::from(coords.x);
        self.tar.z = isize::from(coords.z);
    }

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
