use crate::camera::{CameraSettings, Point};
use crate::grid::GRID_SIZE;
use crate::player::Player;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub fn debug(cam: &mut CameraSettings, top_down_camera_toggle: bool, player: &mut Player) {
    draw_grid_ex(
        100,
        GRID_SIZE,
        RED,
        BLUE,
        vec3(GRID_SIZE / 2.0, 0.0, GRID_SIZE / 2.0),
        Quat::IDENTITY,
    );

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

    if top_down_camera_toggle {
        cam.pos = Point::new(player.x_pos, 10, player.z_pos);
        cam.up = Point::new(0, 0, 1);
        cam.tar = Point::new(player.x_pos, 0, player.z_pos);
        return;
    }

    match get_last_key_pressed() {
        None => {}
        Some(KeyCode::J) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.pos.x -= 1;
            } else {
                cam.pos.x += 1;
            }
        }
        Some(KeyCode::K) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.pos.y -= 1;
            } else {
                cam.pos.y += 1;
            }
        }
        Some(KeyCode::L) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.pos.z -= 1;
            } else {
                cam.pos.z += 1;
            }
        }
        Some(KeyCode::U) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.tar.x -= 1;
            } else {
                cam.tar.x += 1;
            }
        }
        Some(KeyCode::I) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.tar.y -= 1;
            } else {
                cam.tar.y += 1;
            }
        }
        Some(KeyCode::O) => {
            if is_key_down(KeyCode::LeftShift) {
                cam.tar.z -= 1;
            } else {
                cam.tar.z += 1;
            }
        }
        _ => {}
    };
}
