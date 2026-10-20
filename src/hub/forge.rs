use egui_macroquad::egui::{self, Ui};
use shared::ItemRef;

use crate::{
    hub::{
        Filter, GRID_SPACING, TARGET_GRID_SIZE, TOOLBAR_HEIGHT,
        slot::{HoverOnSlot, inventory_slot},
    },
    profile::Profile,
    styles::header_text,
};

pub struct Forge {
    pub slot_1: Option<ItemRef>,
    pub slot_2: Option<ItemRef>,
    forge_btn_unlocked: bool,
}

impl Forge {
    pub const fn new() -> Self {
        Forge {
            slot_1: None,
            slot_2: None,
            forge_btn_unlocked: false,
        }
    }

    pub fn draw_forge(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let filter_id = ui.id().with("inventory_filter");
        let mut filter = ui.data_mut(|d| d.get_temp::<Filter>(filter_id).unwrap_or(Filter::All));

        // toolbar
        ui.vertical(|ui| {
            ui.set_max_height(TOOLBAR_HEIGHT);
            ui.set_min_height(TOOLBAR_HEIGHT);
            ui.label(header_text("Forge"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut filter, Filter::All, "All");
                ui.radio_value(&mut filter, Filter::Pickaxes, "Pickaxes");
                ui.radio_value(&mut filter, Filter::Hats, "Hats");
                ui.radio_value(&mut filter, Filter::Backpacks, "Backpacks");
            });
        });

        ui.data_mut(|d| {
            d.insert_temp(filter_id, filter);
        });

        // main content
        let frame = egui::Frame::new().inner_margin(30.0);
        ui.horizontal_top(|ui| {
            frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(30.0);
                    self.forge_crafting(ui, profile);
                    ui.add_space(20.0);

                    let btn = egui::Button::new("Forge!");
                    if self.slot_1.is_some() && self.slot_2.is_some() {
                        self.forge_btn_unlocked = true;
                    } else {
                        self.forge_btn_unlocked = false;
                    }
                    if ui.add_enabled(self.forge_btn_unlocked, btn).clicked() {
                        self.forge_items(profile);
                    }

                    ui.add_space(20.0);
                    ui.label("Input 1: blah");
                    ui.label("Input 2: blah");
                    ui.label("Result: blah");
                    ui.label("Result stats: blah");
                });
            });
            frame.show(ui, |ui| {
                self.forge_inventory(ui, profile, filter);
            });
        });
    }

    fn forge_crafting(&mut self, ui: &mut Ui, profile: &mut Profile) {
        egui::Grid::new("crafting").num_columns(2).show(ui, |ui| {
            if let Some(item) = self.slot_1 {
                if inventory_slot(ui, 100.0, item.name(), HoverOnSlot::RemoveFromForge).clicked() {
                    self.slot_1 = self.slot_2;
                    self.slot_2 = None;
                }
            } else {
                inventory_slot(ui, 100.0, "", HoverOnSlot::None);
            }
            ui.end_row();

            ui.centered_and_justified(|ui| ui.label("+"));
            inventory_slot(ui, 100.0, "result", HoverOnSlot::None);
            ui.end_row();

            if let Some(item) = self.slot_2 {
                if inventory_slot(ui, 100.0, item.name(), HoverOnSlot::RemoveFromForge).clicked() {
                    self.slot_2 = None;
                }
            } else {
                inventory_slot(ui, 100.0, "", HoverOnSlot::None);
            }
        });
    }

    fn forge_inventory(&mut self, ui: &mut Ui, profile: &mut Profile, filter: Filter) {
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

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("")
                    .spacing(egui::vec2(GRID_SPACING, GRID_SPACING))
                    .show(ui, |ui| {
                        for (i, item) in filtered_items.into_iter().enumerate() {
                            if inventory_slot(
                                ui,
                                slot_size,
                                item.name(),
                                HoverOnSlot::PlaceIntoForge,
                            )
                            .clicked()
                            {
                                self.push_to_slot(item);
                            }

                            if (i + 1) % num_cols == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
    }

    fn push_to_slot(&mut self, item: ItemRef) -> Result<(), ()> {
        if self.slot_1.is_none() {
            self.slot_1 = Some(item);
        } else if self.slot_2.is_none() {
            self.slot_2 = Some(item);
        } else {
            return Err(());
        }
        Ok(())
    }

    fn forge_items(&mut self, profile: &mut Profile) {}
}
