mod block;
mod camera;
mod coordinate;
mod map;
mod player;
mod ui;

use crate::level::{
    block::MiningOutcome, camera::Camera, coordinate::MapCoords, map::GameMap, player::Player,
};

use macroquad::prelude::*;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

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
        match get_last_key_pressed() {
            None => {}
            Some(KeyCode::Up) | Some(KeyCode::W) => self.move_player(Direction::Up, shift_held),
            Some(KeyCode::Down) | Some(KeyCode::S) => self.move_player(Direction::Down, shift_held),
            Some(KeyCode::Right) | Some(KeyCode::D) => {
                self.move_player(Direction::Right, shift_held)
            }
            Some(KeyCode::Left) | Some(KeyCode::A) => self.move_player(Direction::Left, shift_held),
            Some(_) => {}
        }
    }

    pub fn move_player(&mut self, direction: Direction, do_mine: bool) {
        let (dx, dz): (isize, isize) = match direction {
            Direction::Left => (1, 0),
            Direction::Right => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };
        let mut move_to: MapCoords = self.player.get_coords().add((dx, dz));
        // if trying to move out of bounds, undo that movement
        if move_to.x > self.game_map.get_width() {
            move_to.x -= 1
        }
        if move_to.z > self.game_map.get_height() {
            move_to.z -= 1
        }
        let move_to = move_to; // remove mutability

        if do_mine {
            let was_mine_successful = self.game_map.mine_block(move_to, 1);
            match was_mine_successful {
                None => {}
                Some(MiningOutcome::Unbreakable) => {}
                Some(MiningOutcome::Damaged) => self.player.spend_energy(),
                Some(MiningOutcome::Destroyed) => self.player.spend_energy(),
                Some(MiningOutcome::Gained(drops)) => {
                    self.player.collect_drops(drops);
                }
            }
        }

        if self.game_map.is_block(move_to) {
            return;
        }
        self.player.move_to(move_to);
        self.camera.point(move_to);
    }
}
