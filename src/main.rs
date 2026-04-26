use crate::debug::*;
use crate::player::*;
use macroquad::prelude::*;

pub mod debug;
pub mod player;

#[macroquad::main("Mining Game")]
async fn main() {
    let mut player = Player::new(0.0, 0.0, 0.0);

    let mut set = CameraSettings {
        cam_pos_1: 10.0,
        cam_pos_2: 10.0,
        cam_pos_3: 10.0,
        cam_up_1: 0.0,
        cam_up_2: 1.0,
        cam_up_3: 0.0,
        cam_tar_1: 0.0,
        cam_tar_2: 0.0,
        cam_tar_3: 0.0,
    };

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        clear_background(GRAY);

        debug(&mut set, &mut player);

        set_camera(&Camera3D {
            position: vec3(set.cam_pos_1, set.cam_pos_2, set.cam_pos_3),
            up: vec3(set.cam_up_1, set.cam_up_2, set.cam_up_3),
            target: vec3(set.cam_tar_1, set.cam_tar_2, set.cam_tar_3),
            ..Default::default()
        });

        draw_grid(20, 1.0, RED, BLUE);

        player.handle_input();
        player.draw();

        next_frame().await
    }
}
