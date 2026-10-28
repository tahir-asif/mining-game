use egui_macroquad::egui::{self, Ui};
use shared::{ItemId, items::get_item};

use crate::{
    hub::{
        Filter,
        inventory_ui::{
            SIDE_PANEL_WIDTH, draw_filter_bar, draw_inventory_grid, heading_area, hub_panel,
            inventory_layout, item_info_panel,
        },
        slot::HoverOnSlot,
    },
    profile::Profile,
};

pub struct Inventory {
    filter: Filter,
    sell_mode: bool,
    hovered_over_item: Option<ItemId>,
}

impl Inventory {
    pub const fn new() -> Self {
        Inventory {
            filter: Filter::All,
            sell_mode: false,
            hovered_over_item: None,
        }
    }

    pub fn reset_screen(&mut self) {
        self.filter = Filter::All;
        self.sell_mode = false;
        self.hovered_over_item = None;
    }

    pub fn draw(&mut self, ui: &mut Ui, profile: &mut Profile) {
        heading_area(ui, "Inventory", |ui| {
            draw_filter_bar(ui, &mut self.filter);

            let sell_button = if self.sell_mode {
                egui::Button::new("Sell Mode")
                    .fill(egui::Color32::DARK_RED)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
            } else {
                egui::Button::new("Sell Mode")
            };

            if ui.add(sell_button).clicked() {
                self.sell_mode = !self.sell_mode;
            }
        });

        let info = self.hovered_over_item.and_then(get_item);

        ui.horizontal_top(|ui| {
            let side_height = ui.available_height().max(0.0);

            ui.allocate_ui_with_layout(
                egui::vec2(SIDE_PANEL_WIDTH, side_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    item_info_panel(ui, SIDE_PANEL_WIDTH, side_height, info);
                },
            );
            ui.add_space(15.0);

            let grid_width = ui.available_width().max(0.0);
            let grid_height = ui.available_height().max(0.0);
            hub_panel(ui, grid_width, Some(grid_height), |ui| {
                self.inventory_grid(ui, profile);
            });
        });
    }

    fn inventory_grid(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let (num_cols, slot_size) = inventory_layout(ui);
        let hover = if self.sell_mode {
            HoverOnSlot::Sell
        } else {
            HoverOnSlot::Equip
        };

        let visible = |id| self.filter.matches(id);
        let (clicked, hovered) =
            draw_inventory_grid(ui, profile.items(), visible, slot_size, num_cols, hover);

        if let Some(id) = clicked {
            let Some(item) = get_item(id) else { return };
            if self.sell_mode {
                profile.sell_item(item);
            } else {
                profile.equip(item);
            }
        }

        self.hovered_over_item = hovered;
    }
}
