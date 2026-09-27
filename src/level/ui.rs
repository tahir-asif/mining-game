use crate::constants::WINDOW_WIDTH_FLOAT;
use crate::level::player::Player;

use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub fn draw_ui(player: &mut Player) {
    root_ui().label(
        vec2(WINDOW_WIDTH_FLOAT - 100.0, 30.0),
        &format!("ENERGY: {0}", player.energy),
    );
    root_ui().label(
        vec2(WINDOW_WIDTH_FLOAT - 100.0, 10.0),
        &format!("GOLD: {0}", player.gold),
    );
}
