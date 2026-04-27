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
    let mut player = Player::new(0.0, 0.0);
    let mut debug_toggle = false;
    let mut cam = CameraSettings {
        pos: vec3(cam_distance, cam_distance, cam_distance),
        tar: vec3(0.0, 0.0, 0.0),
    };

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }
        if is_key_pressed(KeyCode::Enter) {
            debug_toggle = !debug_toggle;
        }
        if debug_toggle {
            debug(&mut cam, &mut player);
        }

        set_camera(&Camera3D {
            position: cam.pos,
            up: vec3(0.0, 1.0, 0.0),
            target: cam.tar,
            ..Default::default()
        });

        clear_background(GRAY);
        grid();

        player.handle_input(&mut cam);
        player.draw();

        next_frame().await
    }
}
