// TODO: Implement fog of war

mod camera;
mod constants;
mod debug;
mod grid;
mod player;
mod ui;

use crate::camera::{CameraSettings, Point};
use crate::constants::{CAM_DISTANCE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::grid::GameMap;
use crate::player::Player;

use macroquad::prelude::*;

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
    let mut player = Player {
        x: 1,
        z: 1,
        energy: 90,
    };
    let mut camera = CameraSettings {
        pos: Point::new(
            (player.x as isize) - CAM_DISTANCE,
            CAM_DISTANCE,
            (player.z as isize) - CAM_DISTANCE,
        ),
        up: Point::new(0, 1, 0),
        tar: Point::new(player.x as isize, 0, player.z as isize),
    };
    let mut game_map = GameMap::new(10, 10);

    // initilise game
    game_map.generate_level();

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        camera.set();

        game_map.draw();

        player.handle_input(&mut camera, &mut game_map);
        player.draw();

        ui::draw_ui(&mut player);

        debug::debug_controls(
            &mut debug_toggle,
            &mut game_map,
            &mut camera,
            &mut top_down_camera_toggle,
            &mut player,
        );

        next_frame().await
    }
}
