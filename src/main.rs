mod common;
mod constants;
mod hub;
mod level;
mod main_menu;

use crate::{
    common::GameState,
    constants::{WINDOW_HEIGHT_INT, WINDOW_WIDTH_INT},
    hub::hub_update,
    level::Level,
    main_menu::{load_save_update, main_menu_update, settings_update},
};

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Puzzle Miner".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH_INT,
        window_height: WINDOW_HEIGHT_INT,
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

    let mut game_state = GameState::MainMenu;
    game_state = GameState::Level; // skip straight to level for dev

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        game_state = match game_state {
            GameState::MainMenu => main_menu_update(),
            GameState::Settings => settings_update(),
            GameState::LoadSaves => load_save_update(),
            GameState::Hub => hub_update(),
            GameState::Level => level.level_update(&mut debug_toggle, &mut top_down_camera_toggle),
        };

        next_frame().await
    }
}
