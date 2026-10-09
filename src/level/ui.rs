use crate::level::player::Player;

use egui_macroquad::egui::{self, Align2};

pub fn draw_ui(player: &mut Player) {
    egui_macroquad::ui(|ctx| {
        egui::Area::new("HUD_stats".into())
            .anchor(Align2::LEFT_TOP, egui::vec2(-10.0, 10.0))
            .show(ctx, |ui| {
                ui.label(format!("ENERGY: {}", player.energy));
                ui.label(format!("GOLD: {}", player.collected_gold))
            });
    });
}
