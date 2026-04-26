use crate::debug::*;
use crate::player::*;
use macroquad::prelude::*;

pub mod debug;
pub mod player;

#[macroquad::main("Mining Game")]
async fn main() {
    let mut player = Player::new(0.5, 0.5);
    let mut debug_toggle = false;

    let mut cam = CameraSettings {
        pos_x: 10.0,
        pos_y: 10.0,
        pos_z: 10.0,
        up_x: 0.0,
        up_y: 1.0,
        up_z: 0.0,
        tar_x: 0.0,
        tar_y: 0.0,
        tar_z: 0.0,
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
            position: vec3(cam.pos_x, cam.pos_y, cam.pos_z),
            up: vec3(cam.up_x, cam.up_y, cam.up_z),
            target: vec3(cam.tar_x, cam.tar_y, cam.tar_z),
            ..Default::default()
        });

        clear_background(GRAY);

        draw_grid(20, 1.0, RED, BLUE);

        player.handle_input(&mut cam);
        player.draw();

        next_frame().await
    }
}
