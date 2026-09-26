mod block;
mod camera;
mod coordinate;
mod map;
mod player;
mod ui;

use crate::level::{
    camera::Camera,
    map::GameMap,
    player::{Action, Player},
};

use macroquad::prelude::*;

pub struct Level {
    player: Player,
    camera: Camera,
    game_map: GameMap,
}

impl Level {
    pub fn new() -> Self {
        let mut player = Player::new();
        let camera = Camera::new(&mut player);
        let game_map = GameMap::new(10, 10);

        Level {
            player,
            camera,
            game_map,
        }
    }

    pub fn init(&mut self) {
        self.game_map.generate_level();
    }

    pub fn game_loop(&mut self) {
        self.camera.set();
        self.game_map.draw();
        self.player.draw();
        ui::draw_ui(&mut self.player);
    }

    pub fn handle_input(&mut self) {
        let shift_held: bool = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let ctrl_held: bool =
            is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

        match get_last_key_pressed() {
            None => {}
            Some(KeyCode::Up) | Some(KeyCode::W) => self.player.handle_input(
                &mut self.game_map,
                &mut self.camera,
                Action::Up,
                shift_held,
            ),
            Some(KeyCode::Down) | Some(KeyCode::S) => self.player.handle_input(
                &mut self.game_map,
                &mut self.camera,
                Action::Down,
                shift_held,
            ),
            Some(KeyCode::Right) | Some(KeyCode::D) => self.player.handle_input(
                &mut self.game_map,
                &mut self.camera,
                Action::Right,
                shift_held,
            ),
            Some(KeyCode::Left) | Some(KeyCode::A) => self.player.handle_input(
                &mut self.game_map,
                &mut self.camera,
                Action::Left,
                shift_held,
            ),
            Some(_) => {}
        }
    }
}
