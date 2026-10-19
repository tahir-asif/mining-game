use crate::level::{levels::Goal, player::Player};

use egui_macroquad::egui::{self, Align2, Ui};
pub fn draw_ui(player: &mut Player, goal: Goal) {
    egui_macroquad::ui(|ctx| {
        egui::Area::new("hud_top_left".into())
            .anchor(Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
            .show(ctx, |ui| {
                hud_top_left(ui, player);
            });
        egui::Area::new("hud_top_right".into())
            .anchor(Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .show(ctx, |ui| {
                hud_top_right(ui, goal);
            });
    });
}

fn hud_top_right(ui: &mut Ui, goal: Goal) {
    egui::Frame::new()
        .fill(egui::Color32::from_gray(70))
        .inner_margin(8.0)
        .corner_radius(4.0)
        .show(ui, |ui| {
            ui.label(goal.to_str());
        });
}

fn hud_top_left(ui: &mut Ui, player: &mut Player) {
    egui::Frame::new()
        .fill(egui::Color32::from_gray(70))
        .inner_margin(8.0)
        .corner_radius(4.0)
        .show(ui, |ui| {
            egui::Grid::new("hud_stats_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    ui.label("ENERGY");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", player.energy));
                    });
                    ui.end_row();

                    ui.label("TECH");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", player.tech));
                    });
                    ui.end_row();

                    ui.label("GOLD");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", player.collected_gold));
                    });
                    ui.end_row();
                });
        });
}
