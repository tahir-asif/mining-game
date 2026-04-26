use crate::player::Player;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub mod player;

struct CameraSettings {
    cam_pos_1: f32,
    cam_pos_2: f32,
    cam_pos_3: f32,
    cam_up_1: f32,
    cam_up_2: f32,
    cam_up_3: f32,
    cam_tar_1: f32,
    cam_tar_2: f32,
    cam_tar_3: f32,
}

#[macroquad::main("Mining Game")]
async fn main() {
    let mut player = Player {
        x_pos: screen_width() / 2.0,
        y_pos: screen_height() / 2.0,
    };

    let mut set = CameraSettings {
        cam_pos_1: 0.0,
        cam_pos_2: 0.0,
        cam_pos_3: 0.0,
        cam_up_1: 0.0,
        cam_up_2: 0.0,
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

        debug(&mut set);

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

fn debug(set: &mut CameraSettings) {
    if is_key_pressed(KeyCode::Y) {
        set.cam_pos_1 += 10.0;
    }
    if is_key_pressed(KeyCode::U) {
        set.cam_pos_2 += 10.0;
    }
    if is_key_pressed(KeyCode::I) {
        set.cam_pos_3 += 10.0;
    }
    if is_key_pressed(KeyCode::H) {
        set.cam_up_1 += 10.0;
    }
    if is_key_pressed(KeyCode::J) {
        set.cam_up_1 += 10.0;
    }
    if is_key_pressed(KeyCode::L) {
        set.cam_up_1 += 10.0;
    }
    if is_key_pressed(KeyCode::B) {
        set.cam_tar_1 += 10.0;
    }
    if is_key_pressed(KeyCode::N) {
        set.cam_tar_1 += 10.0;
    }
    if is_key_pressed(KeyCode::M) {
        set.cam_tar_1 += 10.0;
    }

    root_ui().window(hash!(), vec2(0.0, 0.0), vec2(100.0, 110.0), |ui| {
        ui.label(vec2(5.0, 1.0), &format!("pos[1] = {0}", set.cam_pos_1));
        ui.label(vec2(5.0, 11.0), &format!("pos[2] = {0}", set.cam_pos_2));
        ui.label(vec2(5.0, 21.0), &format!("pos[3] = {0}", set.cam_pos_3));
        ui.label(vec2(5.0, 31.0), &format!("up[1]  = {0}", set.cam_up_1));
        ui.label(vec2(5.0, 41.0), &format!("up[2]  = {0}", set.cam_up_2));
        ui.label(vec2(5.0, 51.0), &format!("up[3]  = {0}", set.cam_up_3));
        ui.label(vec2(5.0, 61.0), &format!("tar[1] = {0}", set.cam_tar_1));
        ui.label(vec2(5.0, 71.0), &format!("tar[2] = {0}", set.cam_tar_2));
        ui.label(vec2(5.0, 81.0), &format!("tar[3] = {0}", set.cam_tar_3));
    });
}
