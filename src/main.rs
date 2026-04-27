use crate::camera::*;
use crate::debug::*;
use crate::grid::*;
use crate::player::*;

use macroquad::prelude::*;

pub mod camera;
pub mod debug;
pub mod grid;
pub mod player;

#[macroquad::main("Mining Game")]
async fn main() {
    let cam_distance = 10;
    let mut debug_toggle = false;
    let mut top_down_camera_toggle = false;
    let mut player = Player::new(0, 0);
    let mut game_map = GameMap { blocks: vec![] };
    game_map.grid();
    let mut camera = CameraSettings {
        pos: Point::new(-cam_distance, cam_distance, -cam_distance),
        up: Point::new(0, 1, 0),
        tar: Point::new(0, 0, 0),
    };

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        camera.set();

        game_map.draw();

        player.handle_input(&mut camera, &mut game_map);
        player.draw();

        if is_key_pressed(KeyCode::Enter) {
            debug_toggle = !debug_toggle;
        }
        if is_key_pressed(KeyCode::P) {
            // if toggling it off, reset camera
            if top_down_camera_toggle {
                camera.pos = Point::new(
                    player.x_pos - cam_distance,
                    cam_distance,
                    player.z_pos - cam_distance,
                );
                camera.up = Point::new(0, 1, 0);
            }
            top_down_camera_toggle = !top_down_camera_toggle;
        }
        if debug_toggle {
            debug(&mut camera, top_down_camera_toggle, &mut player);
        }

        next_frame().await
    }
}
