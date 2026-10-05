mod block;
mod camera;
pub mod coordinate;
mod debug;
pub mod levels;
mod map;
pub mod outcome;
mod player;
mod ui;
pub mod winnings;

use crate::{
    game::TransitionState,
    level::{
        block::MiningOutcome,
        camera::Camera,
        coordinate::MapCoords,
        levels::{Goal, LevelData, LevelId},
        map::GameMap,
        outcome::Outcome,
        player::Player,
        winnings::Winnings,
    },
};

use macroquad::prelude::*;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Level {
    player: Player,
    camera: Camera,
    game_map: GameMap,
    meta_data: LevelData,
}

impl Level {
    pub fn new(id: LevelId, starting_energy: u16, starting_tech: u16, mining_power: u16) -> Self {
        let meta_data = LevelData {
            id,
            spawn: MapCoords::new(1, 1),
            goal: Goal::Collect(MapCoords::new(9, 9)),
            prerequisies: &[],
        };
        let mut game_map = GameMap::new(10, 10);
        let mut player = Player::new(
            meta_data.spawn,
            starting_energy,
            starting_tech,
            mining_power,
        );
        let camera = Camera::new(&mut player);

        game_map.generate_level();

        Level {
            player,
            camera,
            game_map,
            meta_data,
        }
    }

    pub fn update(
        &mut self,
        debug_toggle: &mut bool,
        top_down_camera_toggle: &mut bool,
    ) -> TransitionState {
        self.camera.set();
        self.game_map.draw();
        self.player.draw();
        ui::draw_ui(&mut self.player);
        self.debug(debug_toggle, top_down_camera_toggle);
        self.handle_input();
        if self.is_goal_achieved() {
            return TransitionState::EndLevel(Outcome::Win(self.calc_winnings()));
        }
        if self.player.energy == 0 {
            return TransitionState::EndLevel(Outcome::Lose);
        }
        if get_last_key_pressed() == Some(KeyCode::Escape) {
            return TransitionState::EndLevel(Outcome::Exit);
        }
        TransitionState::None
    }

    fn is_goal_achieved(&self) -> bool {
        match self.meta_data.goal {
            Goal::Collect(coords) => self.player.get_coords() == coords,
            Goal::MineAllGold => false,
        }
    }

    fn calc_winnings(&self) -> Winnings {
        Winnings {
            gold: self.player.collected_gold,
        }
    }

    fn debug(&mut self, debug_toggle: &mut bool, top_down_camera_toggle: &mut bool) {
        debug::debug_controls(
            debug_toggle,
            &mut self.game_map,
            &mut self.camera,
            top_down_camera_toggle,
            &mut self.player,
        );
    }

    fn handle_input(&mut self) {
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

    fn move_player(&mut self, direction: Direction, do_mine: bool) {
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
            let was_mine_successful = self.game_map.mine_block(move_to, self.player.mining_power);
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
