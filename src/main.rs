mod constants;
// mod debug;
mod level;

use crate::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    level::Level,
};

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
    // declare "global" variables; settings
    let mut debug_toggle = false;
    let mut top_down_camera_toggle = false;

    let mut level = Level::new();
    level.init();

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        level.game_loop();
        level.handle_input();

        // debug::debug_controls(
        //     &mut debug_toggle,
        //     &mut game_map,
        //     &mut camera,
        //     &mut top_down_camera_toggle,
        //     &mut player,
        // );

        next_frame().await
    }
}
