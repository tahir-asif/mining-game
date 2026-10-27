pub mod coordinate;
pub mod levels;
pub mod outcome;
pub mod winnings;

mod abilities;
mod block;
mod camera;
mod map;
mod player;
mod ui;

use crate::{
    game::TransitionState,
    level::{
        abilities::{BOMB_COST, Bomb},
        block::{DropKind, MiningOutcome},
        camera::Camera,
        coordinate::MapCoords,
        levels::{Goal, LevelData, LevelId, get_level},
        map::GameMap,
        outcome::Outcome,
        player::Player,
        ui::{HudAction, SelectedAbility},
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
    gold_blocks_mined: u16,
    ability: SelectedAbility,
    placed_bomb: Option<Bomb>,
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

        game_map.generate_level(meta_data.block_ids, meta_data.spawn);

        Level {
            player,
            camera,
            game_map,
            meta_data,
            is_paused: false,
            gold_blocks_mined: 0,
            ability: SelectedAbility::None,
            placed_bomb: None,
        }
    }

    pub fn update(&mut self) -> TransitionState {
        self.camera.set();
        self.game_map.draw();
        self.player.draw();

        if get_last_key_pressed() == Some(KeyCode::Escape) {
            self.ability = SelectedAbility::None;
            self.is_paused = !self.is_paused;
        }
        if self.is_paused {
            return self.pause();
        }

        if self.player.energy == 0 {
            return self.loss_screen();
        }
        if self.is_goal_achieved() {
            return self.win_screen();
        }

        self.draw_ui_and_get_input();
        self.handle_input();
        self.tick_bombs();

        TransitionState::None
    }

    fn draw_ui_and_get_input(&mut self) {
        let action = ui::draw_ui(
            &self.player,
            self.meta_data.goal,
            self.gold_blocks_mined,
            self.ability,
        );

        match action {
            HudAction::None => {}
            HudAction::Toggle(clicked) if self.ability == clicked => {
                self.ability = SelectedAbility::None;
            }
            HudAction::Toggle(clicked) => self.ability = clicked,
            HudAction::Deselect => self.ability = SelectedAbility::None,
        }
    }

    fn is_goal_achieved(&self) -> bool {
        // TODO: Change collect goal to detect mining GoalChest instead of moving to certain coords
        match self.meta_data.goal {
            Goal::Collect(coords) => self.player.get_coords() == coords,
            Goal::MineAllGold(total) => self.gold_blocks_mined == total,
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
            stars: self.player.stars,
        }
    }

    fn tick_bombs(&mut self) {
        let Some(bomb) = self.placed_bomb.as_mut() else {
            return;
        };
        if let Some(blocks_to_mine) = bomb.bomb_update() {
            for block in blocks_to_mine {
                self.mine(block, u16::MAX, false);
                self.game_map.reveal_map(block, map::Reveal::Bomb);
            }
            self.placed_bomb = None;
        }
    }

    fn handle_input(&mut self) {
        let is_shift_held: bool =
            is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        match get_last_key_pressed() {
            None => {}
            Some(KeyCode::Up) | Some(KeyCode::W) => {
                self.handle_input_direction(Direction::Up, is_shift_held);
            }
            Some(KeyCode::Down) | Some(KeyCode::S) => {
                self.handle_input_direction(Direction::Down, is_shift_held)
            }
            Some(KeyCode::Right) | Some(KeyCode::D) => {
                self.handle_input_direction(Direction::Right, is_shift_held)
            }
            Some(KeyCode::Left) | Some(KeyCode::A) => {
                self.handle_input_direction(Direction::Left, is_shift_held)
            }
            Some(_) => self.ability = SelectedAbility::None,
        }
    }

    fn handle_input_direction(&mut self, direction: Direction, is_shift_held: bool) {
        let (dx, dz): (i8, i8) = match direction {
            Direction::Left => (1, 0),
            Direction::Right => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };

        let mut current = self.player.get_coords();
        let new_coords: MapCoords = current.add((dx, dz));

        // TODO: holding shift while placing makes bomb not appear, but still explodes
        match self.ability {
            SelectedAbility::None => self.move_player(new_coords, is_shift_held),
            SelectedAbility::Bomb => self.place_bomb(new_coords),
        }
    }

    fn place_bomb(&mut self, coords: MapCoords) {
        // TODO: Possibly disable placing bomb on certain blocks
        if self.placed_bomb.is_some() || self.player.tech < BOMB_COST {
            self.ability = SelectedAbility::None;
            return;
        }
        let bomb = Bomb::new(coords, self.game_map.is_block(coords));
        self.placed_bomb = Some(bomb);
        self.player.tech -= BOMB_COST;
        self.ability = SelectedAbility::None;
    }

    fn move_player(&mut self, coords: MapCoords, do_mine: bool) {
        if do_mine {
            self.mine(coords, self.player.mining_power, true);
        }
        if self.game_map.is_block(coords) {
            return;
        }
        self.player.move_to(coords);
        self.game_map.reveal_map(coords, map::Reveal::Player);
        self.camera.point(coords);
    }

    fn mine(&mut self, block: MapCoords, mining_power: u16, do_spend_energy: bool) {
        let was_mine_successful = self.game_map.mine_block(block, mining_power);
        match was_mine_successful {
            None => {}
            Some(MiningOutcome::Unbreakable) => {}
            Some(MiningOutcome::Damaged) | Some(MiningOutcome::Destroyed) => {
                if do_spend_energy {
                    self.player.spend_energy();
                }
            }
            Some(MiningOutcome::Gained(drops)) => {
                if let Some(drop_kind) = self.player.collect_drops(drops) {
                    match drop_kind {
                        DropKind::Gold => {
                            self.gold_blocks_mined += 1;
                        }
                        DropKind::Item => {
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
                        DropKind::Star => {}
                        DropKind::Energy => {}
                        DropKind::Tech => {}
                    }
                }
            }
        }
    }

    fn loss_screen(&mut self) -> TransitionState {
        let mut transition = TransitionState::None;

        egui_macroquad::ui(|ctx| {
            let fill_colour = egui::Color32::from_rgb(255, 0, 0).gamma_multiply(0.2);
            let frame = egui::Frame::new().fill(fill_colour);

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(screen_height() * 0.25);

                    ui.label(egui::RichText::new("DEFEAT").size(64.0));
                    ui.add_space(20.0);

                    let button_size = egui::vec2(250.0, 50.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Hub"))
                        .clicked()
                    {
                        transition = TransitionState::EndLevel(Outcome::Lose);
                    }
                });
            });
        });

        transition
    }

    fn win_screen(&mut self) -> TransitionState {
        let mut transition = TransitionState::None;

        egui_macroquad::ui(|ctx| {
            let fill_colour = egui::Color32::from_rgb(0, 255, 0).gamma_multiply(0.2);
            let frame = egui::Frame::new().fill(fill_colour);

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(screen_height() * 0.25);

                    ui.label(egui::RichText::new("VICTORY!").size(64.0));
                    ui.add_space(20.0);

                    let button_size = egui::vec2(250.0, 50.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Hub"))
                        .clicked()
                    {
                        transition = TransitionState::EndLevel(Outcome::Win(self.calc_winnings()));
                    }
                });
            });
        });

        transition
    }

    fn pause(&mut self) -> TransitionState {
        let mut transition = TransitionState::None;

        egui_macroquad::ui(|ctx| {
            let fill_colour = egui::Color32::from_rgb(60, 60, 60).gamma_multiply(0.7);
            let frame = egui::Frame::new().fill(fill_colour);

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
}
