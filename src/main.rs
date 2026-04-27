use crate::debug::*;
use crate::grid::*;
use crate::player::*;
use macroquad::prelude::*;

pub mod debug;
pub mod grid;
pub mod player;

#[macroquad::main("Mining Game")]
async fn main() {
    let cam_distance = GRID_SIZE * 10.0;
    let mut debug_toggle = false;
    let mut top_down_camera_toggle = false;
    let mut player = Player::new(0, 0);
    let mut game_map = GameMap { blocks: vec![] };
    game_map.grid();
    let mut cam = CameraSettings {
        pos: vec3(-cam_distance, cam_distance, -cam_distance),
        up: vec3(0.0, 1.0, 0.0),
        tar: vec3(0.0, 0.0, 0.0),
    };

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        set_camera(&Camera3D {
            position: cam.pos,
            up: cam.up,
            target: cam.tar,
            ..Default::default()
        });

        clear_background(GRAY);
        game_map.draw();

        player.handle_input(&mut cam, &mut game_map);
        player.draw();

        if is_key_pressed(KeyCode::Enter) {
            debug_toggle = !debug_toggle;
        }
        if is_key_pressed(KeyCode::P) {
            if top_down_camera_toggle {
                // if toggling it off, reset camera
                cam.pos = vec3(-cam_distance, cam_distance, -cam_distance);
                cam.up = vec3(0.0, 1.0, 0.0);
            }
            top_down_camera_toggle = !top_down_camera_toggle;
        }
        if debug_toggle {
            debug(&mut cam, top_down_camera_toggle, &mut player);
        }

        next_frame().await
    }
}
