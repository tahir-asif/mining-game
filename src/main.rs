use crate::camera::*;
use crate::constants::*;
use crate::debug::*;
use crate::grid::*;
use crate::player::*;

use macroquad::prelude::*;

pub mod camera;
pub mod constants;
pub mod debug;
pub mod grid;
pub mod player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mining Game".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // declare "global" variables
    let mut debug_toggle = false;
    let mut top_down_camera_toggle = false;

    // create objects
    let mut player = Player { x: 0, z: 0 };
    let mut game_map = GameMap::new(10, 10);
    game_map.generate_level();
    let mut camera = CameraSettings {
        pos: Point::new(-CAM_DISTANCE, CAM_DISTANCE, -CAM_DISTANCE),
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

        debug_controls(
            &mut debug_toggle,
            &mut game_map,
            &mut camera,
            &mut top_down_camera_toggle,
            &mut player,
        );

        next_frame().await
    }
}
