use crate::camera::{CameraSettings, Point};
use crate::constants::{CAM_DISTANCE, GRID_SIZE};
use crate::grid::GameMap;
use crate::player::Player;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub fn debug_controls(
    debug_toggle: &mut bool,
    game_map: &mut GameMap,
    camera: &mut CameraSettings,
    top_down_camera_toggle: &mut bool,
    player: &mut Player,
) {
    if is_key_pressed(KeyCode::Enter) {
        *debug_toggle = !*debug_toggle;
    }

    if is_key_pressed(KeyCode::P) {
        // if toggling off top down mode, reset camera
        if *top_down_camera_toggle {
            camera.pos = Point::new(
                player.x.cast_signed() - CAM_DISTANCE,
                CAM_DISTANCE,
                player.z.cast_signed() - CAM_DISTANCE,
            );
            camera.up = Point::new(0, 1, 0);
        }
        *top_down_camera_toggle = !*top_down_camera_toggle;
    }

    if *debug_toggle {
        debug(game_map, camera, *top_down_camera_toggle, player);
    }
}

fn debug(
    game_map: &mut GameMap,
    cam: &mut CameraSettings,
    top_down_camera_toggle: bool,
    player: &mut Player,
) {
    grid(game_map);

    window_overlay(cam, player);

    debug_camera(cam, player, top_down_camera_toggle);
}

fn scale(unsigned_int: usize) -> f32 {
    (unsigned_int as f32) * GRID_SIZE
}

fn grid(game_map: &mut GameMap) {
    draw_line_3d(
        vec3(0.0, 0.0, 0.0),
        vec3(scale(game_map.height), 0.0, 0.0),
        RED,
    );
    draw_line_3d(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, scale(game_map.height)),
        RED,
    );
    draw_line_3d(
        vec3(scale(game_map.height), 0.0, 0.0),
        vec3(scale(game_map.width), 0.0, scale(game_map.height)),
        RED,
    );
    draw_line_3d(
        vec3(0.0, 0.0, scale(game_map.height)),
        vec3(scale(game_map.width), 0.0, scale(game_map.height)),
        RED,
    );
    for i in 1..game_map.width {
        draw_line_3d(
            vec3(scale(i), 0.0, 0.0),
            vec3(scale(i), 0.0, scale(game_map.height)),
            BLUE,
        );
    }
    for i in 1..game_map.height {
        draw_line_3d(
            vec3(0.0, 0.0, scale(i)),
            vec3(scale(game_map.width), 0.0, scale(i)),
            BLUE,
        );
    }
}

fn window_overlay(cam: &mut CameraSettings, player: &mut Player) {
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
        ui.label(vec2(5.0, 51.0), &format!("({0}, {1})", player.x, player.z));
    });
}

fn debug_camera(cam: &mut CameraSettings, player: &mut Player, top_down_camera_toggle: bool) {
    if top_down_camera_toggle {
        cam.pos = Point::new(player.x.cast_signed(), 10, player.z.cast_signed());
        cam.up = Point::new(0, 0, 1);
        cam.tar = Point::new(player.x.cast_signed(), 0, player.z.cast_signed());
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
