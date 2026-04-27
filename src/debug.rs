use crate::player::Player;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub struct CameraSettings {
    pub pos: Vec3,
    pub tar: Vec3,
}

pub fn debug(cam: &mut CameraSettings, player: &mut Player) {
    root_ui().window(hash!(), vec2(1.0, 1.0), vec2(150.0, 80.0), |ui| {
        // camera position
        ui.label(vec2(5.0, 1.0), "Camera Position");
        ui.label(
            vec2(5.0, 11.0),
            &format!("({0}, {1}, {2})", cam.pos.x, cam.pos.y, cam.pos.z),
        );

        // camera target
        ui.label(vec2(5.0, 21.0), "Camera Target");
        ui.label(
            vec2(5.0, 31.0),
            &format!("({0}, {1}, {2})", cam.tar.x, cam.tar.y, cam.tar.z),
        );

        // player position
        ui.label(vec2(5.0, 41.0), "Player Position");
        ui.label(
            vec2(5.0, 51.0),
            &format!("({0}, {1})", player.x_pos, player.z_pos),
        );
    });
}
