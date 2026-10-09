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
        let frame = egui::Frame::new().fill(egui::Color32::TRANSPARENT);

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Puzzle Miner");
                    if ui.button("Play").clicked() {
                        transition =
                            TransitionState::GoTo(GameState::LoadSave(LoadSaveState::new()));
                    }
                    if ui.button("Settings").clicked() {
                        transition = TransitionState::GoTo(GameState::Settings);
                    }
                    if ui.button("Quit").clicked() {
                        transition = TransitionState::Quit;
                    }
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
        let frame = egui::Frame::new().fill(egui::Color32::TRANSPARENT);

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Main Menu").clicked() {
                        transition = TransitionState::GoTo(GameState::MainMenu);
                    }
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
                    }
                });
            });
    });

    transition
}
