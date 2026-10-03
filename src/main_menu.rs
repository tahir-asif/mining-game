use crate::common::GameState;
use macroquad::{color::*, ui::root_ui, window::clear_background};

pub fn main_menu_update() -> GameState {
    clear_background(MAROON);
    if root_ui().button(None, "Play") {
        return GameState::LoadSaves;
    }
    if root_ui().button(None, "Settings") {
        return GameState::Settings;
    }
    GameState::MainMenu
}

pub fn settings_update() -> GameState {
    clear_background(BROWN);
    if root_ui().button(None, "< Main Menu") {
        return GameState::MainMenu;
    }
    GameState::Settings
}

pub fn load_save_update() -> GameState {
    clear_background(BEIGE);
    if root_ui().button(None, "Save 1") {
        return GameState::Hub;
    }
    if root_ui().button(None, "Save 2") {
        return GameState::Hub;
    }
    GameState::LoadSaves
}
