use egui_macroquad::egui::{self, Ui};

use crate::{
    hub::{
        Filter, GRID_SPACING, TARGET_GRID_SIZE, TOOLBAR_HEIGHT,
        slot::{HoverOnSlot, inventory_slot},
    },
    profile::Profile,
    styles::header_text,
};

pub fn draw_inventory(ui: &mut Ui, profile: &mut Profile) {
    let filter_id = ui.id().with("inventory_filter");
    let mut filter = ui.data_mut(|d| d.get_temp::<Filter>(filter_id).unwrap_or(Filter::All));

    let sell_id = ui.id().with("inventory_sell_mode");
    let mut sell_mode = ui.data_mut(|d| d.get_temp::<bool>(sell_id).unwrap_or(false));

    // toolbar
    ui.vertical(|ui| {
        ui.set_max_height(TOOLBAR_HEIGHT);
        ui.set_min_height(TOOLBAR_HEIGHT);
        ui.label(header_text("Inventory"));
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            // filters
            ui.radio_value(&mut filter, Filter::All, "All");
            ui.radio_value(&mut filter, Filter::Pickaxes, "Pickaxes");
            ui.radio_value(&mut filter, Filter::Hats, "Hats");
            ui.radio_value(&mut filter, Filter::Backpacks, "Backpacks");

            // sell button
            let sell_button = if sell_mode {
                egui::Button::new("Sell Mode")
                    .fill(egui::Color32::DARK_RED)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
            } else {
                egui::Button::new("Sell Mode")
            };

            if ui.add(sell_button).clicked() {
                sell_mode = !sell_mode;
            }
        });
    });

    ui.data_mut(|d| {
        d.insert_temp(filter_id, filter);
        d.insert_temp(sell_id, sell_mode);
    });

    // main content
    let frame = egui::Frame::new().inner_margin(30.0);
    frame.show(ui, |ui| inventory_grid(ui, profile, filter, sell_mode));
}

fn inventory_grid(ui: &mut Ui, profile: &mut Profile, filter: Filter, sell_mode: bool) {
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
    let slot_size = (available_width - (num_cols as f32 - 1.0) * GRID_SPACING) / num_cols as f32;

    let mut hover = HoverOnSlot::Equip;
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("inventory-grid")
            .spacing(egui::vec2(GRID_SPACING, GRID_SPACING))
            .show(ui, |ui| {
                for (i, item) in filtered_items.into_iter().enumerate() {
                    if sell_mode {
                        hover = HoverOnSlot::Sell;
                    }

                    if inventory_slot(ui, slot_size, item.name(), hover).clicked() {
                        if sell_mode {
                            profile.sell(item);
                        } else {
                            profile.equip(item);
                        }
                    }

                    if (i + 1) % num_cols == 0 {
                        ui.end_row();
                    }
                }
            });
    });
}
