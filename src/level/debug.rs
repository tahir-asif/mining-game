use crate::constants::GRID_SIZE;
use crate::level::camera::Camera;
use crate::level::coordinate::CameraCoords;
use crate::level::map::GameMap;
use crate::level::player::Player;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub fn debug_controls(
    debug_toggle: &mut bool,
    game_map: &mut GameMap,
    camera: &mut Camera,
    top_down_camera_toggle: &mut bool,
    player: &mut Player,
) {
    if is_key_pressed(KeyCode::Enter) {
        *debug_toggle = !*debug_toggle;
    }

    if is_key_pressed(KeyCode::P) {
        if *top_down_camera_toggle {
            camera.reset(player);
        }
        *top_down_camera_toggle = !*top_down_camera_toggle;
    }

    if *debug_toggle {
        debug(game_map, camera, *top_down_camera_toggle, player);
    }
}

fn debug(
    game_map: &mut GameMap,
    cam: &mut Camera,
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
        vec3(scale(game_map.get_height() - 1), 0.0, 0.0),
        RED,
    );
    draw_line_3d(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, scale(game_map.get_height() - 1)),
        RED,
    );
    draw_line_3d(
        vec3(scale(game_map.get_height() - 1), 0.0, 0.0),
        vec3(
            scale(game_map.get_width() - 1),
            0.0,
            scale(game_map.get_height() - 1),
        ),
        RED,
    );
    draw_line_3d(
        vec3(0.0, 0.0, scale(game_map.get_height() - 1)),
        vec3(
            scale(game_map.get_width() - 1),
            0.0,
            scale(game_map.get_height() - 1),
        ),
        RED,
    );
    for i in 1..game_map.get_width() - 1 {
        draw_line_3d(
            vec3(scale(i), 0.0, 0.0),
            vec3(scale(i), 0.0, scale(game_map.get_height() - 1)),
            BLUE,
        );
    }
    for i in 1..game_map.get_height() - 1 {
        draw_line_3d(
            vec3(0.0, 0.0, scale(i)),
            vec3(scale(game_map.get_width() - 1), 0.0, scale(i)),
            BLUE,
        );
    }
}

fn window_overlay(cam: &mut Camera, player: &mut Player) {
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
        ui.label(vec2(5.0, 51.0), &player.get_coords().to_string());
    });
}

fn debug_camera(cam: &mut Camera, player: &mut Player, top_down_camera_toggle: bool) {
    if top_down_camera_toggle {
        cam.pos = CameraCoords::new(0, 10, 0);
        cam.up = CameraCoords::new(0, 0, 1);
        cam.tar = CameraCoords::new(0, 0, 0);
        cam.point(player.get_coords());
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
