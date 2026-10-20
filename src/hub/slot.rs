use egui_macroquad::egui::{self, Ui};

#[derive(PartialEq, Clone, Copy)]
pub enum HoverOnSlot {
    None,
    Sell,
    Equip,
    PlaceIntoForge,
    RemoveFromForge,
}

pub fn inventory_slot(
    ui: &mut Ui,
    size: f32,
    item_name: &str,
    hover: HoverOnSlot,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let mut visuals = *ui.style().interact(&response);

        if hover == HoverOnSlot::Sell {
            visuals.bg_stroke = egui::Stroke::new(2.0, egui::Color32::RED);
        }

        let text_color = if hover == HoverOnSlot::Sell {
            egui::Color32::LIGHT_RED
        } else {
            visuals.text_color()
        };
        let corner_radius = visuals.corner_radius;
        let bg_fill = visuals.bg_fill;
        let bg_stroke = visuals.bg_stroke;

        let painter = ui.painter().with_clip_rect(rect);

        painter.rect(
            rect,
            corner_radius,
            bg_fill,
            bg_stroke,
            egui::StrokeKind::Inside,
        );

        let padding = 4.0;
        let image_rect = rect.shrink(padding);
        painter.rect_filled(image_rect, 4.0, egui::Color32::from_gray(40));

        let text_rect = rect.shrink(padding);
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(text_rect)
                .layout(egui::Layout::bottom_up(egui::Align::Center)),
        );

        let text = egui::RichText::new(item_name)
            .text_style(egui::TextStyle::Small)
            .color(text_color);

        child_ui.add(egui::Label::new(text).wrap().selectable(false));

        if response.hovered() {
            match hover {
                HoverOnSlot::None => {}
                HoverOnSlot::Sell => {
                    painter.text(
                        image_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "SELL?",
                        egui::FontId::proportional(20.0),
                        egui::Color32::RED,
                    );
                }
                HoverOnSlot::Equip => {
                    painter.text(
                        image_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Equip?",
                        egui::FontId::proportional(20.0),
                        egui::Color32::GOLD,
                    );
                }
                HoverOnSlot::PlaceIntoForge => {
                    painter.text(
                        image_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Forge?",
                        egui::FontId::proportional(20.0),
                        egui::Color32::DARK_GREEN,
                    );
                }
                HoverOnSlot::RemoveFromForge => {
                    painter.text(
                        image_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Remove?",
                        egui::FontId::proportional(20.0),
                        egui::Color32::DARK_RED,
                    );
                }
            }
        }
    }

    response
}
