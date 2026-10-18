use egui_macroquad::egui::{self, Ui};

use crate::{save::Profile, styles::header_text};

const GRID_SPACING: f32 = 8.0;
const TARGET_GRID_SIZE: f32 = 120.0;

#[derive(PartialEq, Clone, Copy)]
enum Filter {
    All,
    Pickaxes,
    Hats,
    Backpacks,
}

pub fn draw_inventory(ui: &mut Ui, profile: &mut Profile) {
    ui.vertical(|ui| {
        ui.label(header_text("Inventory"));

        let filter_id = ui.id().with("inventory_filter");
        let mut filter = ui.data_mut(|d| d.get_temp::<Filter>(filter_id).unwrap_or(Filter::All));
        ui.horizontal(|ui| {
            ui.radio_value(&mut filter, Filter::All, "All");
            ui.radio_value(&mut filter, Filter::Pickaxes, "Pickaxes");
            ui.radio_value(&mut filter, Filter::Hats, "Hats");
            ui.radio_value(&mut filter, Filter::Backpacks, "Backpacks");
        });

        ui.data_mut(|d| d.insert_temp(filter_id, filter));

        let filtered_items: Vec<_> = profile
            .items
            .iter()
            .filter(|item| match filter {
                Filter::All => true,
                Filter::Pickaxes => item.is_pickaxe(),
                Filter::Hats => item.is_hat(),
                Filter::Backpacks => item.is_backpack(),
            })
            .copied()
            .collect();

        let available_width = ui.available_width();
        let num_cols = ((available_width + GRID_SPACING) / (TARGET_GRID_SIZE + GRID_SPACING))
            .floor()
            .max(1.0) as usize;
        let slot_size =
            (available_width - (num_cols as f32 - 1.0) * GRID_SPACING) / num_cols as f32;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("inventory-grid")
                .spacing(egui::vec2(GRID_SPACING, GRID_SPACING))
                .show(ui, |ui| {
                    for (i, item) in filtered_items.into_iter().enumerate() {
                        if inventory_slot(ui, slot_size, item.name()).clicked() {
                            profile.equip(item);
                        }

                        if (i + 1) % num_cols == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    });
}

fn inventory_slot(ui: &mut Ui, size: f32, item_name: &str) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let text_color = visuals.text_color();
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
    }

    response
}
