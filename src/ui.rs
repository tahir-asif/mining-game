use crate::constants;
use crate::player::Player;

use macroquad::prelude::*;
use macroquad::ui::root_ui;

const WINDOW_WIDTH: f32 = constants::WINDOW_WIDTH as f32;
const WINDOW_HEIGHT: f32 = constants::WINDOW_HEIGHT as f32;

pub fn draw_ui(player: &mut Player) {
    root_ui().label(
        vec2(10.0, WINDOW_HEIGHT - 20.0),
        &format!("ENERGY: {0}", player.energy),
    );
}
