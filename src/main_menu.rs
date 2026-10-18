use crate::{
    constants::SAVE_SLOT_COUNT,
    game::{GameState, TransitionState},
};
use egui_macroquad::egui;
use macroquad::{color::*, window::clear_background};

pub fn main_menu_update() -> TransitionState {
    let mut transition = TransitionState::None;
    clear_background(MAROON);

    egui_macroquad::ui(|egui_ctx| {
        let frame = egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(30.0);

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Puzzle Miner").size(64.0));
                    ui.add_space(30.0);

                    ui.scope(|ui| {
                        ui.style_mut().text_styles.insert(
                            egui::TextStyle::Button,
                            egui::FontId::new(32.0, egui::FontFamily::Proportional),
                        );

                        let button_size = egui::vec2(260.0, 40.0);
                        let btn = ui.add_sized(button_size, egui::Button::new("Play"));
                        if btn.clicked() {
                            transition =
                                TransitionState::GoTo(GameState::LoadSave(LoadSaveState::new()));
                        }
                        ui.add_space(24.0);
                        let btn = ui.add_sized(button_size, egui::Button::new("Settings"));
                        if btn.clicked() {
                            transition = TransitionState::GoTo(GameState::Settings);
                        }
                        ui.add_space(24.0);
                        let btn = ui.add_sized(button_size, egui::Button::new("Quit"));
                        if btn.clicked() {
                            transition = TransitionState::Quit;
                        }
                    });
                });
            });
    });

    transition
}

pub fn settings_update() -> TransitionState {
    let mut transition = TransitionState::None;
    clear_background(BROWN);

    egui_macroquad::ui(|egui_ctx| {
        let frame = egui::Frame::new().fill(egui::Color32::TRANSPARENT);

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Main Menu").clicked() {
                        transition = TransitionState::GoTo(GameState::MainMenu);
                    }
                });
            });
    });

    transition
}

pub struct SlotInput {
    pub name: String,
}
pub struct LoadSaveState {
    pub slots: Vec<SlotInput>,
}

impl LoadSaveState {
    pub fn new() -> Self {
        Self {
            slots: (0..SAVE_SLOT_COUNT)
                .map(|_| SlotInput {
                    name: String::new(),
                })
                .collect(),
        }
    }
}

pub fn load_save_update(state: &mut LoadSaveState) -> TransitionState {
    let mut transition = TransitionState::None;
    clear_background(BEIGE);

    egui_macroquad::ui(|egui_ctx| {
        let frame = egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(20.0);

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Main Menu").clicked() {
                        transition = TransitionState::GoTo(GameState::MainMenu);
                    }
                    ui.add_space(30.0);
                    for (i, slot) in state.slots.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Slot {i}"));
                            ui.text_edit_singleline(&mut slot.name);
                            if ui.button("Save?").clicked() && !slot.name.trim().is_empty() {
                                transition = TransitionState::LoadProfile {
                                    slot: i + 1,
                                    name: slot.name.clone(),
                                };
                            };
                        });
                        ui.add_space(10.0);
                    }
                });
            });
    });

    transition
}
