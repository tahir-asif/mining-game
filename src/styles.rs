use std::sync::Arc;

use egui::Color32;
use egui_macroquad::egui::{
    self, FontData, FontDefinitions, FontFamily, FontId, RichText, TextStyle,
};

#[rustfmt::skip]
pub fn setup_global_style(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        style.visuals.override_text_color = Some(Color32::WHITE);

        style.text_styles = [
            (TextStyle::Small,   FontId::new(16.0, FontFamily::Proportional)),
            (TextStyle::Body,    FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Button,  FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(32.0, FontFamily::Proportional)),
        ]
        .into_iter()
        .collect();

        style.spacing.button_padding = egui::vec2(20.0, 3.0);

        let button_rounding = egui::CornerRadius::same(8);
        let button_bg = egui::Color32::from_rgb(45, 45, 65);

        // Normal, un-interacted state
        style.visuals.widgets.inactive.weak_bg_fill = button_bg;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::GRAY);
        style.visuals.widgets.inactive.corner_radius = button_rounding;

        // Hovered state
        style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(65, 65, 90);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
        style.visuals.widgets.hovered.corner_radius = button_rounding;

        // Active (pressed) state
        style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(80, 80, 120);
        style.visuals.widgets.active.corner_radius = button_rounding;
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

pub fn header_text(t: impl Into<String>) -> RichText {
    RichText::new(t).text_style(TextStyle::Heading)
}

pub fn normal_text(t: impl Into<String>) -> RichText {
    RichText::new(t).text_style(TextStyle::Body)
}

pub fn small_text(t: impl Into<String>) -> RichText {
    RichText::new(t).text_style(TextStyle::Small)
}
