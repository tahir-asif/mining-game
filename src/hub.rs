use crate::{
    game::{GameState, TransitionState},
    level::levels::LEVELS,
    save::Profile,
};
use egui_macroquad::egui;
use macroquad::{color::*, window::clear_background};

pub fn hub_update(profile: &mut Profile) -> TransitionState {
    let mut transition = TransitionState::None;
    clear_background(VIOLET);

    egui_macroquad::ui(|egui_ctx| {
        let frame = egui::Frame::new().fill(egui::Color32::TRANSPARENT);

        egui::SidePanel::left("SideBar")
            .frame(frame)
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Main Menu").clicked() {
                        transition = TransitionState::GoTo(GameState::MainMenu);
                    }

                    ui.label("Inventory");

                    let label = format!("Gold: {}", profile.gold);
                    ui.label(&label);

                    let label = format!(
                        "Equipped: {}, {}, {}",
                        profile.equipped.pickaxe.name,
                        profile.equipped.backpack.name,
                        profile.equipped.hat.name
                    );
                    ui.label(&label);

                    ui.label("Items:");
                    for item in &profile.items {
                        ui.label(format!("{item}"));
                    }
                });
            });
        egui::SidePanel::right("Levels")
            .frame(frame)
            .show(egui_ctx, |ui| {
                for level in LEVELS {
                    let label = format!("Level {}", level.id + 1);
                    if ui.button(&label).clicked() {
                        transition = TransitionState::StartLevel(level.id);
                    }
                }
            });
    });

    transition
}
