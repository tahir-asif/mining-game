use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::warn;
use shared::items::{ItemId, get_item};

use crate::{
    hub::{
        Filter, TOOLBAR_HEIGHT,
        inventory_ui::{draw_filter_bar, draw_inventory_grid, inventory_layout},
        slot::{HoverOnSlot, inventory_slot},
    },
    profile::Profile,
    styles::header_text,
};

pub struct Forge {
    pub slot_1: Option<ItemId>,
    pub slot_2: Option<ItemId>,
    pub result_slot: Option<ItemId>,
    forge_btn_unlocked: bool,
    filter: Filter,
}

#[derive(Clone, Copy)]
enum Slot {
    First,
    Second,
}

impl Forge {
    pub const fn new() -> Self {
        Forge {
            slot_1: None,
            slot_2: None,
            result_slot: None,
            forge_btn_unlocked: false,
            filter: Filter::All,
        }
    }

    pub fn reset_screen(&mut self) {
        self.slot_1 = None;
        self.slot_2 = None;
        self.result_slot = None;
        self.forge_btn_unlocked = false;
        self.filter = Filter::All;
    }

    pub fn draw_forge(&mut self, ui: &mut Ui, profile: &mut Profile) {
        ui.vertical(|ui| {
            ui.set_max_height(TOOLBAR_HEIGHT);
            ui.set_min_height(TOOLBAR_HEIGHT);
            ui.label(header_text("Forge"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                draw_filter_bar(ui, &mut self.filter);
            });
        });

        let frame = egui::Frame::new().inner_margin(30.0);
        ui.horizontal_top(|ui| {
            frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(30.0);
                    self.forge_crafting(ui, profile);
                    ui.add_space(20.0);

                    self.forge_btn_unlocked = self.slot_1.is_some()
                        && self.slot_2.is_some()
                        && self.result_slot.is_none();
                    let btn = egui::Button::new("Forge!");
                    if ui.add_enabled(self.forge_btn_unlocked, btn).clicked() {
                        self.forge_items();
                    }

                    ui.add_space(20.0);
                    ui.label("Input 1: blah");
                    ui.label("Input 2: blah");
                    ui.label("Result: blah");
                    ui.label("Result stats: blah");
                });
            });
            frame.show(ui, |ui| {
                self.forge_inventory(ui, profile);
            });
        });
    }

    fn forge_inventory(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let (num_cols, slot_size) = inventory_layout(ui);

        let visible = |id| match self.slot_1 {
            Some(s1) => id == s1,
            None => self.filter.matches(id),
        };

        if let Some(id) = draw_inventory_grid(
            ui,
            profile.items(),
            visible,
            slot_size,
            num_cols,
            HoverOnSlot::PlaceIntoForge,
        ) {
            let _ = self.push_to_slot(id, profile);
        }
    }

    fn forge_crafting(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let mut remove_first = false;
        let mut remove_second = false;

        egui::Grid::new("crafting").num_columns(2).show(ui, |ui| {
            if let Some(id) = self.slot_1 {
                if inventory_slot(ui, 100.0, item_name(id), HoverOnSlot::RemoveFromForge).clicked()
                {
                    remove_first = true;
                }
            } else {
                inventory_slot(ui, 100.0, "", HoverOnSlot::None);
            }
            ui.end_row();

            ui.centered_and_justified(|ui| ui.label("+"));
            if let Some(id) = self.result_slot {
                if inventory_slot(ui, 100.0, item_name(id), HoverOnSlot::CollectFromForge).clicked()
                {
                    profile.add_item(id);
                    self.result_slot = None;
                }
            } else {
                inventory_slot(ui, 100.0, "result", HoverOnSlot::None);
            }
            ui.end_row();

            if let Some(id) = self.slot_2 {
                if inventory_slot(ui, 100.0, item_name(id), HoverOnSlot::RemoveFromForge).clicked()
                {
                    remove_second = true;
                }
            } else {
                inventory_slot(ui, 100.0, "", HoverOnSlot::None);
            }
        });

        if remove_first {
            self.return_slot(Slot::First, profile);
            self.slot_1 = self.slot_2.take();
        } else if remove_second {
            self.return_slot(Slot::Second, profile);
        }
    }

    fn push_to_slot(&mut self, id: ItemId, profile: &mut Profile) -> Result<(), ()> {
        if self.slot_1.is_none() {
            self.slot_1 = Some(id);
        } else if self.slot_2.is_none() {
            self.slot_2 = Some(id);
        } else {
            return Err(());
        }
        profile.remove_item(id);
        Ok(())
    }

    fn return_slot(&mut self, which: Slot, profile: &mut Profile) {
        let taken = match which {
            Slot::First => self.slot_1.take(),
            Slot::Second => self.slot_2.take(),
        };
        if let Some(id) = taken {
            profile.add_item(id);
        }
    }

    pub fn return_all(&mut self, profile: &mut Profile) {
        self.return_slot(Slot::Second, profile);
        self.return_slot(Slot::First, profile);
        if let Some(id) = self.result_slot.take() {
            profile.add_item(id);
        }
    }

    fn forge_items(&mut self) {
        let (Some(a), Some(b)) = (self.slot_1, self.slot_2) else {
            return;
        };
        if a != b {
            return;
        }
        let Some(result_id) = recipe_for(a) else {
            return;
        };
        self.slot_1 = None;
        self.slot_2 = None;
        self.result_slot = Some(result_id);
    }
}

/// Utility to name crafting slots in forge.
/// Intentionaly returns "" if id doesn't exist.
fn item_name(id: ItemId) -> &'static str {
    get_item(id).map_or("", |i| i.name())
}

fn recipe_for(id: ItemId) -> Option<ItemId> {
    warn!("Recipes not implemented, returning same item");
    Some(id)
}
