use crate::player::Player;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub struct CameraSettings {
    pub cam_pos_1: f32,
    pub cam_pos_2: f32,
    pub cam_pos_3: f32,
    pub cam_up_1: f32,
    pub cam_up_2: f32,
    pub cam_up_3: f32,
    pub cam_tar_1: f32,
    pub cam_tar_2: f32,
    pub cam_tar_3: f32,
}

pub fn debug(set: &mut CameraSettings, player: &mut Player) {
    if is_key_pressed(KeyCode::Y) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_pos_1 -= 10.0
        } else {
            set.cam_pos_1 += 10.0
        }
    }
    if is_key_pressed(KeyCode::U) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_pos_2 -= 10.0
        } else {
            set.cam_pos_2 += 10.0
        }
    }
    if is_key_pressed(KeyCode::I) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_pos_3 -= 10.0
        } else {
            set.cam_pos_3 += 10.0
        }
    }
    if is_key_pressed(KeyCode::H) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_up_1 -= 1.0
        } else {
            set.cam_up_1 += 1.0
        }
    }
    if is_key_pressed(KeyCode::J) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_up_2 -= 1.0
        } else {
            set.cam_up_2 += 1.0
        }
    }
    if is_key_pressed(KeyCode::K) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_up_3 -= 1.0
        } else {
            set.cam_up_3 += 1.0
        }
    }
    if is_key_pressed(KeyCode::B) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_tar_1 -= 1.0
        } else {
            set.cam_tar_1 += 1.0
        }
    }
    if is_key_pressed(KeyCode::N) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_tar_2 -= 1.0
        } else {
            set.cam_tar_2 += 1.0
        }
    }
    if is_key_pressed(KeyCode::M) {
        if is_key_down(KeyCode::LeftShift) {
            set.cam_tar_3 -= 1.0
        } else {
            set.cam_tar_3 += 1.0
        }
    }

    // X-axis (Red)
    draw_line_3d(vec3(0., 0., 0.), vec3(500., 0., 0.), RED);
    // Y-axis (Green)
    draw_line_3d(vec3(0., 0., 0.), vec3(0., 500., 0.), GREEN);
    // Z-axis (Blue)
    draw_line_3d(vec3(0., 0., 0.), vec3(0., 0., 500.), BLUE);

    root_ui().window(hash!(), vec2(0.0, 0.0), vec2(100.0, 150.0), |ui| {
        ui.label(vec2(5.0, 1.0), &format!("pos[1] = {0}", set.cam_pos_1));
        ui.label(vec2(5.0, 11.0), &format!("pos[y] = {0}", set.cam_pos_2));
        ui.label(vec2(5.0, 21.0), &format!("pos[3] = {0}", set.cam_pos_3));
        ui.label(vec2(5.0, 31.0), &format!("up[1]  = {0}", set.cam_up_1));
        ui.label(vec2(5.0, 41.0), &format!("up[2]  = {0}", set.cam_up_2));
        ui.label(vec2(5.0, 51.0), &format!("up[3]  = {0}", set.cam_up_3));
        ui.label(vec2(5.0, 61.0), &format!("tar[1] = {0}", set.cam_tar_1));
        ui.label(vec2(5.0, 71.0), &format!("tar[2] = {0}", set.cam_tar_2));
        ui.label(vec2(5.0, 81.0), &format!("tar[3] = {0}", set.cam_tar_3));

        ui.label(vec2(5.0, 91.0), &format!("x = {0}", player.x_pos));
        ui.label(vec2(5.0, 101.0), &format!("y = {0}", player.y_pos));
        ui.label(vec2(5.0, 111.0), &format!("z = {0}", player.z_pos));
    });
}
