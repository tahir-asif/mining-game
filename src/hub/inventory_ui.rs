use egui_macroquad::egui::{self, Ui};

use crate::hub::{
    Filter, GRID_SPACING, TARGET_GRID_SIZE,
    slot::{HoverOnSlot, inventory_slot},
};
use shared::items::{get_item, ItemId};

pub fn draw_filter_bar(ui: &mut Ui, filter: &mut Filter) {
    ui.radio_value(filter, Filter::All, "All");
    ui.radio_value(filter, Filter::Pickaxes, "Pickaxes");
    ui.radio_value(filter, Filter::Hats, "Hats");
    ui.radio_value(filter, Filter::Backpacks, "Backpacks");
}

pub fn inventory_layout(ui: &Ui) -> (usize, f32) {
    let available_width = ui.available_width();
    let num_cols = ((available_width + GRID_SPACING) / (TARGET_GRID_SIZE + GRID_SPACING))
        .floor()
        .max(1.0) as usize;
    let slot_size = (available_width - (num_cols as f32 - 1.0) * GRID_SPACING) / num_cols as f32;
    (num_cols, slot_size)
}

pub fn draw_inventory_grid(
    ui: &mut Ui,
    items: &[ItemId],
    visible: impl Fn(ItemId) -> bool,
    slot_size: f32,
    num_cols: usize,
    hover: HoverOnSlot,
) -> Option<ItemId> {
    let mut clicked = None;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new(ui.id().with("inv_grid"))
                .spacing(egui::vec2(GRID_SPACING, GRID_SPACING))
                .show(ui, |ui| {
                    let mut visible_count = 0;

                    for &id in items {
                        if !visible(id) {
                            continue;
                        }

                        let name = get_item(id).map_or("", |i| i.name());

                        if inventory_slot(ui, slot_size, name, hover).clicked() {
                            clicked = Some(id);
                        }

                        visible_count += 1;
                        if visible_count % num_cols == 0 {
                            ui.end_row();
                        }
                    }
                });
        });

    clicked
}
