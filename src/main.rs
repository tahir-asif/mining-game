use crate::camera::*;
use crate::debug::*;
use crate::grid::*;
use crate::player::*;

use macroquad::prelude::*;

pub mod camera;
pub mod debug;
pub mod grid;
pub mod player;

const CAM_DISTANCE: i32 = 10;

#[macroquad::main("Mining Game")]
async fn main() {
    // declare "global" variables
    let mut debug_toggle = false;
    let mut top_down_camera_toggle = false;

    // create objects
    let mut player = Player::new(0, 0);
    let mut game_map: GameMap = Default::default();
    let mut camera = CameraSettings {
        pos: Point::new(-CAM_DISTANCE, CAM_DISTANCE, -CAM_DISTANCE),
        up: Point::new(0, 1, 0),
        tar: Point::new(0, 0, 0),
    };

    // initilise game map
    game_map.grid();

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        camera.set();

        game_map.draw();

        player.handle_input(&mut camera, &mut game_map);
        player.draw();

        debug_controls(
            &mut camera,
            &mut top_down_camera_toggle,
            &mut player,
            &mut debug_toggle,
        );

        next_frame().await
    }
}

fn debug_controls(
    camera: &mut CameraSettings,
    top_down_camera_toggle: &mut bool,
    player: &mut Player,
    debug_toggle: &mut bool,
) {
    if is_key_pressed(KeyCode::Enter) {
        *debug_toggle = !*debug_toggle;
    }

    if is_key_pressed(KeyCode::P) {
        // if toggling off top down mode, reset camera
        if *top_down_camera_toggle {
            camera.pos = Point::new(
                player.x_pos - CAM_DISTANCE,
                CAM_DISTANCE,
                player.z_pos - CAM_DISTANCE,
            );
            camera.up = Point::new(0, 1, 0);
        }
        *top_down_camera_toggle = !*top_down_camera_toggle;
    }

    if *debug_toggle {
        debug(camera, *top_down_camera_toggle, player);
    }
}
