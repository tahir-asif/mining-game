mod constants;
mod game;
mod hub;
mod items;
mod level;
mod main_menu;
mod save;

use crate::{
    constants::{WINDOW_HEIGHT_INT, WINDOW_WIDTH_INT},
    game::{Game, GameState, TransitionState},
    hub::hub_update,
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
    let mut game = Game::new();

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            break; // end game
        }

        let transition_state: TransitionState = match game.get_state() {
            GameState::MainMenu => main_menu_update(),
            GameState::Settings => settings_update(),
            GameState::LoadSave(state) => load_save_update(state),
            GameState::Hub => hub_update(game.get_profile()),
            GameState::Level(level) => level.update(&mut debug_toggle, &mut top_down_camera_toggle),
        };

        game.set_state(transition_state);

        next_frame().await
    }
}
