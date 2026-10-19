mod constants;
mod game;
mod hub;
mod level;
mod main_menu;
mod save;
mod styles;

use crate::{
    constants::{WINDOW_HEIGHT_INT, WINDOW_WIDTH_INT},
    game::{Game, GameState, TransitionState},
    main_menu::{load_save_update, main_menu_update, settings_update},
    styles::{setup_custom_fonts, setup_global_style},
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
    let mut game = Game::new();
    let mut is_quit = false;

    // ui setup
    egui_macroquad::cfg(|ctx| {
        setup_global_style(ctx);
        setup_custom_fonts(ctx);
    });

    // main game loop
    loop {
        if is_key_down(KeyCode::LeftSuper) & is_key_pressed(KeyCode::W) {
            is_quit = true;
        }

        let transition_state: TransitionState = match game.get_state() {
            GameState::MainMenu => main_menu_update(),
            GameState::Settings => settings_update(),
            GameState::LoadSave(state) => load_save_update(state),
            GameState::Hub => game.hub_update(),
            GameState::Level(level) => level.update(),
            GameState::Quit => {
                is_quit = true;
                TransitionState::None
            }
        };

        game.set_state(transition_state);

        if is_quit {
            break;
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
