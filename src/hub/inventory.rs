use egui_macroquad::egui::{self, Ui};
use shared::items::get_item;

use crate::{
    hub::{
        Filter, TOOLBAR_HEIGHT,
        inventory_ui::{draw_filter_bar, draw_inventory_grid, inventory_layout},
        slot::HoverOnSlot,
    },
    profile::Profile,
    styles::header_text,
};

pub struct Inventory {
    filter: Filter,
    sell_mode: bool,
}

impl Inventory {
    pub const fn new() -> Self {
        Inventory {
            filter: Filter::All,
            sell_mode: false,
        }
    }

    pub fn reset_screen(&mut self) {
        self.filter = Filter::All;
        self.sell_mode = false;
    }

    pub fn draw(&mut self, ui: &mut Ui, profile: &mut Profile) {
        ui.vertical(|ui| {
            ui.set_max_height(TOOLBAR_HEIGHT);
            ui.set_min_height(TOOLBAR_HEIGHT);
            ui.label(header_text("Inventory"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
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
        });

        let frame = egui::Frame::new().inner_margin(30.0);
        frame.show(ui, |ui| self.inventory_grid(ui, profile));
    }

    fn inventory_grid(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let (num_cols, slot_size) = inventory_layout(ui);
        let hover = if self.sell_mode {
            HoverOnSlot::Sell
        } else {
            HoverOnSlot::Equip
        };

        let visible = |id| self.filter.matches(id);
        if let Some(id) =
            draw_inventory_grid(ui, profile.items(), visible, slot_size, num_cols, hover)
        {
            let Some(item) = get_item(id) else { return };
            if self.sell_mode {
                profile.sell_item(item);
            } else {
                profile.equip(item);
            }
        }
    }
}
