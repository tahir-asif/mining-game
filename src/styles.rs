use std::sync::Arc;

use egui::Color32;
use egui_macroquad::egui::{self, FontData, FontDefinitions, FontFamily, FontId, TextStyle};

pub fn setup_global_style(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        // text styles
        style.override_text_style = Some(egui::TextStyle::Body);
        style.visuals.override_text_color = Some(Color32::WHITE);

        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(24.0, FontFamily::Proportional));
    });
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "main_font".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/Amarante-Regular.ttf"
        ))),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "main_font".to_owned());

    ctx.set_fonts(fonts);
}
