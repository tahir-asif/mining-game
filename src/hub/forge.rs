use egui_macroquad::egui::Ui;

use crate::styles::header_text;

pub fn draw_forge(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.label(header_text("Forge"));
    });
}
