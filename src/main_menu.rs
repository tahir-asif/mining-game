use crate::{
    constants::SAVE_SLOT_COUNT,
    game::{GameState, TransitionState},
    profile::Profile,
    save,
    styles::small_text,
};
use egui_macroquad::egui::{self, Layout, RichText, Ui};
use macroquad::prelude::*;

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
                ui.vertical_centered(|ui| {
                    ui.add_space(screen_height() * 0.25);

                    ui.label(egui::RichText::new("Puzzle Miner").size(64.0));
                    ui.add_space(24.0);

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

pub struct LoadSaveState {
    pub slots: Vec<Option<Profile>>,
    pub renaming_slot: Option<usize>,
    pub rename_buffer: String,
}

impl LoadSaveState {
    pub fn new() -> Self {
        let mut slots = Vec::with_capacity(SAVE_SLOT_COUNT as usize);
        for i in 0..SAVE_SLOT_COUNT {
            match save::load_profile(i + 1) {
                Ok(p) => slots.push(p),
                Err(e) => {
                    eprintln!("failed to load slot {}: {}", i + 1, e);
                    slots.push(None);
                }
            }
        }
        Self {
            slots,
            renaming_slot: None,
            rename_buffer: String::new(),
        }
    }
}

pub fn load_save_update(state: &mut LoadSaveState) -> TransitionState {
    let mut transition = TransitionState::None;
    clear_background(BEIGE);

    egui_macroquad::ui(|ctx| {
        let frame = egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(10.0);

        egui::TopBottomPanel::top("top-bar")
            .frame(frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let back_btn = ui.button("Back");
                    let btn_width = back_btn.rect.width();
                    if back_btn.clicked() {
                        transition = TransitionState::GoTo(GameState::MainMenu);
                    }

                    let spacing = ui.spacing().item_spacing.x;
                    let title_width = ui.available_width() - btn_width - spacing;

                    ui.allocate_ui_with_layout(
                        egui::vec2(title_width, 0.0),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.label(RichText::new("Save/Load").size(64.0));
                        },
                    );

                    ui.add_space(btn_width + spacing);
                });
            });

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                save_slots(ui, &mut transition, state);
            });
        });
    });

    transition
}

pub fn save_slots(ui: &mut Ui, transition: &mut TransitionState, state: &mut LoadSaveState) {
    for (i, slot_opt) in state.slots.iter_mut().enumerate() {
        let mut deleted = false;

        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_gray(20).gamma_multiply(0.5))
            .show(ui, |ui| {
                ui.set_width(ui.available_width() * 0.9);
                ui.set_height(100.0);

                match slot_opt {
                    None => {
                        egui::Grid::new(format!("empty_header_grid_{}", i))
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label(format!("Slot {}", i + 1));
                                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("Save").clicked() {
                                        *transition = TransitionState::LoadProfile {
                                            slot: (i + 1).try_into().unwrap(),
                                            name: format!("Slot {}", i + 1),
                                        };
                                    }
                                });
                                ui.end_row();
                            });

                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("EMPTY").size(32.0).weak());
                        });
                    }
                    Some(profile) => {
                        if state.renaming_slot == Some(i) {
                            egui::Grid::new(format!("rename_grid_{}", i))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut state.rename_buffer)
                                            .min_size(egui::Vec2 { x: 300.0, y: 0.0 }),
                                    );

                                    ui.with_layout(
                                        Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.button("Confirm").clicked() {
                                                profile.name = state.rename_buffer.clone();
                                                let _ = save::save_profile(profile);
                                                state.renaming_slot = None;
                                            }
                                            if ui.button("Cancel").clicked() {
                                                state.renaming_slot = None;
                                            }
                                        },
                                    );
                                    ui.end_row();
                                });
                        } else {
                            egui::Grid::new(format!("header_grid_{}", i))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.label(&profile.name);

                                    ui.with_layout(
                                        Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.button("Load").clicked() {
                                                *transition = TransitionState::LoadProfile {
                                                    slot: (i + 1).try_into().unwrap(),
                                                    name: profile.name.clone(),
                                                };
                                            }
                                            if ui.button("Delete").clicked() {
                                                save::delete_profile((i + 1).try_into().unwrap());
                                                deleted = true;
                                            }
                                            if ui.button("Rename").clicked() {
                                                state.renaming_slot = Some(i);
                                                state.rename_buffer = profile.name.clone();
                                            }
                                        },
                                    );
                                    ui.end_row();
                                });
                        }

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label(
                                small_text(format!("Gold: {}", profile.gold))
                                    .color(egui::Color32::GOLD),
                            );
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            ui.label(small_text(format!(
                                "Level: {}",
                                profile.latest_completed_level,
                            )));
                        });

                        ui.horizontal(|ui| {
                            ui.label(small_text("Equipped:"));
                            ui.add_space(20.0);
                            ui.label(small_text(format!("{}", profile.equipped.pickaxe.name)));
                            ui.separator();
                            ui.label(small_text(format!("{}", profile.equipped.hat.name)));
                            ui.separator();
                            ui.label(small_text(format!("{}", profile.equipped.backpack.name)));
                        });
                    }
                }
            });
        if deleted {
            *slot_opt = None;
        }
        ui.add_space(20.0);
    }
}
