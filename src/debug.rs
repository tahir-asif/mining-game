use crate::player::Player;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub struct CameraSettings {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub up_x: f32,
    pub up_y: f32,
    pub up_z: f32,
    pub tar_x: f32,
    pub tar_y: f32,
    pub tar_z: f32,
}

pub fn debug(cam: &mut CameraSettings, player: &mut Player) {
    if is_key_pressed(KeyCode::Y) {
        if is_key_down(KeyCode::LeftShift) {
            cam.pos_x -= 10.0
        } else {
            cam.pos_x += 10.0
        }
    }
    if is_key_pressed(KeyCode::U) {
        if is_key_down(KeyCode::LeftShift) {
            cam.pos_y -= 10.0
        } else {
            cam.pos_y += 10.0
        }
    }
    if is_key_pressed(KeyCode::I) {
        if is_key_down(KeyCode::LeftShift) {
            cam.pos_z -= 10.0
        } else {
            cam.pos_z += 10.0
        }
    }
    if is_key_pressed(KeyCode::H) {
        if is_key_down(KeyCode::LeftShift) {
            cam.up_x -= 1.0
        } else {
            cam.up_x += 1.0
        }
    }
    if is_key_pressed(KeyCode::J) {
        if is_key_down(KeyCode::LeftShift) {
            cam.up_y -= 1.0
        } else {
            cam.up_y += 1.0
        }
    }
    if is_key_pressed(KeyCode::K) {
        if is_key_down(KeyCode::LeftShift) {
            cam.up_z -= 1.0
        } else {
            cam.up_z += 1.0
        }
    }
    if is_key_pressed(KeyCode::B) {
        if is_key_down(KeyCode::LeftShift) {
            cam.tar_x -= 1.0
        } else {
            cam.tar_x += 1.0
        }
    }
    if is_key_pressed(KeyCode::N) {
        if is_key_down(KeyCode::LeftShift) {
            cam.tar_y -= 1.0
        } else {
            cam.tar_y += 1.0
        }
    }
    if is_key_pressed(KeyCode::M) {
        if is_key_down(KeyCode::LeftShift) {
            cam.tar_z -= 1.0
        } else {
            cam.tar_z += 1.0
        }
    }

    root_ui().window(hash!(), vec2(0.0, 0.0), vec2(100.0, 150.0), |ui| {
        ui.label(vec2(5.0, 1.0), &format!("pos[1] = {0}", cam.pos_x));
        ui.label(vec2(5.0, 11.0), &format!("pos[y] = {0}", cam.pos_y));
        ui.label(vec2(5.0, 21.0), &format!("pos[3] = {0}", cam.pos_z));
        ui.label(vec2(5.0, 31.0), &format!("up[1]  = {0}", cam.up_x));
        ui.label(vec2(5.0, 41.0), &format!("up[2]  = {0}", cam.up_y));
        ui.label(vec2(5.0, 51.0), &format!("up[3]  = {0}", cam.up_z));
        ui.label(vec2(5.0, 61.0), &format!("tar[1] = {0}", cam.tar_x));
        ui.label(vec2(5.0, 71.0), &format!("tar[2] = {0}", cam.tar_y));
        ui.label(vec2(5.0, 81.0), &format!("tar[3] = {0}", cam.tar_z));

        ui.label(vec2(5.0, 91.0), &format!("x = {0}", player.x_pos));
        ui.label(vec2(5.0, 101.0), &format!("y = {0}", player.y_pos));
        ui.label(vec2(5.0, 111.0), &format!("z = {0}", player.z_pos));
    });
}
