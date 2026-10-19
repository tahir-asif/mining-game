mod block;
mod camera;
pub mod coordinate;
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
        levels::{Goal, LevelData, LevelId, get_level},
        map::GameMap,
        outcome::Outcome,
        player::Player,
        winnings::Winnings,
    },
};

use egui_macroquad::egui;
use macroquad::prelude::*;
use shared::items::get_item;

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
    meta_data: &'static LevelData,
    is_paused: bool,
}

impl Level {
    pub fn new(id: LevelId, starting_energy: u16, starting_tech: u16, mining_power: u16) -> Self {
        let meta_data = get_level(id).unwrap_or_else(|| panic!("Level ID {id} does not exist"));
        let mut game_map = GameMap::new(meta_data.width, meta_data.height);
        let player = Player::new(
            meta_data.spawn,
            starting_energy,
            starting_tech,
            mining_power,
        );
        let camera = Camera::new(player.get_coords());

        game_map.generate_level(meta_data.block_ids);

        Level {
            player,
            camera,
            game_map,
            meta_data,
            is_paused: false,
        }
    }

    pub fn update(&mut self) -> TransitionState {
        self.camera.set();
        self.game_map.draw();
        self.player.draw();

        self.handle_input();

        if get_last_key_pressed() == Some(KeyCode::Escape) {
            self.is_paused = !self.is_paused;
        }
        if self.is_paused {
            return self.pause();
        }

        ui::draw_ui(&mut self.player, self.meta_data.goal);

        if self.player.energy == 0 {
            return TransitionState::EndLevel(Outcome::Lose);
        }
        if self.is_goal_achieved() {
            return TransitionState::EndLevel(Outcome::Win(self.calc_winnings()));
        }

        TransitionState::None
    }

    fn pause(&mut self) -> TransitionState {
        let mut transition = TransitionState::None;

        egui_macroquad::ui(|ctx| {
            let frame = egui::Frame::new().fill(egui::Color32::TRANSPARENT);

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(screen_height() * 0.25);

                    ui.label(egui::RichText::new("PAUSED").size(64.0));
                    ui.add_space(20.0);

                    let button_size = egui::vec2(250.0, 50.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Resume"))
                        .clicked()
                    {
                        self.is_paused = false;
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("Back to Hub"))
                        .clicked()
                    {
                        transition = TransitionState::EndLevel(Outcome::Exit);
                    }
                });
            });
        });

        transition
    }

    fn is_goal_achieved(&self) -> bool {
        match self.meta_data.goal {
            Goal::Collect(coords) => self.player.get_coords() == coords,
            Goal::MineAllGold => false,
        }
    }

    fn calc_winnings(&self) -> Winnings {
        let items = self
            .player
            .collected_items
            .iter()
            .filter_map(|&id| get_item(id))
            .collect();
        Winnings {
            gold: self.player.collected_gold,
            items,
            latest_completed_level: self.meta_data.id,
        }
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
        let (dx, dz): (i8, i8) = match direction {
            Direction::Left => (1, 0),
            Direction::Right => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };

        let mut current = self.player.get_coords();
        let move_to: MapCoords = current.add((dx, dz));

        if do_mine {
            self.mine(move_to);
        }

        if self.game_map.is_block(move_to) {
            return;
        }
        self.player.move_to(move_to);
        self.camera.point(move_to);
    }

    fn mine(&mut self, block: MapCoords) {
        let was_mine_successful = self.game_map.mine_block(block, self.player.mining_power);
        match was_mine_successful {
            None => {}
            Some(MiningOutcome::Unbreakable) => {}
            Some(MiningOutcome::Damaged) => self.player.spend_energy(),
            Some(MiningOutcome::Destroyed) => self.player.spend_energy(),
            Some(MiningOutcome::Gained(drops)) => {
                let chest_loot = self.player.collect_drops(drops);
                if chest_loot {
                    let chest_loot = self
                        .meta_data
                        .chests
                        .iter()
                        .find(|(coord, _)| *coord == block)
                        .map(|(_, id)| id);
                    if let Some(loot) = chest_loot {
                        self.player.collected_items.push(*loot);
                    }
                }
            }
        }
    }
}
