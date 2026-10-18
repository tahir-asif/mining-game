use egui_macroquad::egui::Ui;

use crate::styles::header_text;

pub fn draw_crafting(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.label(header_text("Crafting"));
    });
}
