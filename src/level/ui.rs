use crate::level::{levels::Goal, player::Player};

use egui_macroquad::egui::{self, Align2, Frame, Label, Ui};

pub fn draw_ui(player: &mut Player, goal: Goal, gold_mined: u16) {
    egui_macroquad::ui(|ctx| {
        egui::Area::new("hud_top_left".into())
            .anchor(Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
            .show(ctx, |ui| {
                hud_top_left(ui, player);
            });
        egui::Area::new("hud_top_right".into())
            .anchor(Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .show(ctx, |ui| {
                hud_top_right(ui, player, goal, gold_mined);
            });
        egui::Area::new("hud_bottom_right".into())
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
            .show(ctx, |ui| {
                hud_bottom_right(ui, player);
            });
    });
}

fn hud_frame() -> Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_gray(70))
        .inner_margin(8.0)
        .corner_radius(4.0)
}

fn hud_bottom_right(ui: &mut Ui, player: &mut Player) {
    let frame = hud_frame();

    let block_size = egui::vec2(50.0, 50.0);

    ui.horizontal(|ui| {
        frame.show(ui, |ui| {
            ui.allocate_ui_with_layout(
                block_size,
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.label("TECH");
                    ui.label(format!("{}", player.tech));
                },
            );
        });

        frame.show(ui, |ui| {
            ui.label("Bomb");
            ui.label("10");
        });
    });
}

fn hud_top_right(ui: &mut Ui, player: &mut Player, goal: Goal, gold_mined: u16) {
    let frame = hud_frame();

    ui.horizontal_top(|ui| {
        frame.show(ui, |ui| {
            ui.add(Label::new(goal.to_str(gold_mined)).wrap_mode(egui::TextWrapMode::Extend));
        });
        frame.show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("GOLD");
                ui.label(format!("{}", player.collected_gold));
            });
        });
    });
}

fn hud_top_left(ui: &mut Ui, player: &mut Player) {
    let frame = hud_frame();

    frame.show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.label("ENERGY");
            ui.label(format!("{}", player.energy));
        });
    });
}
