use crate::common::GameState;
use macroquad::{color::*, ui::root_ui, window::clear_background};

pub fn hub_update() -> GameState {
    clear_background(VIOLET);
    if root_ui().button(None, "Level 1") {
        return GameState::Level;
    }
    if root_ui().button(None, "Level 2") {
        return GameState::Level;
    }
    GameState::Hub
}
