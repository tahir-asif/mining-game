use egui_macroquad::egui::{self, Ui};
use shared::items::{ItemId, get_item};

use crate::{
    constants::ITEM_PRICE_TO_FORGE_COST_FACTOR,
    hub::{
        Filter,
        inventory_ui::{
            SIDE_PANEL_WIDTH, draw_filter_bar, draw_inventory_grid, heading_area, hub_panel,
            inventory_layout, item_info_panel, panel_chrome_height, panel_heading, panel_row,
        },
        slot::{HoverOnSlot, inventory_slot},
    },
    profile::Profile,
};

const CRAFTING_SLOT_SIZE: f32 = 100.0;
const CRAFTING_SLOT_GAP: f32 = 8.0;
const CRAFTING_ARROW_HEIGHT: f32 = 28.0;
const CRAFTING_ARROW_GAP: f32 = 8.0;
const CRAFTING_ARROW_COLOR: egui::Color32 = egui::Color32::from_rgb(140, 190, 255);
const CRAFTING_PANEL_GAP: f32 = 15.0;
const CRAFTING_HEADING_GAP: f32 = 10.0;
const CRAFTING_COST_GAP: f32 = 10.0;
const CRAFTING_GRID_GAP: f32 = 10.0;
const CRAFTING_BUTTON_HEIGHT: f32 = 32.0;
const CRAFTING_HEIGHT_SAFETY: f32 = 4.0;

pub struct Forge {
    pub slot_1: Option<ItemId>,
    pub slot_2: Option<ItemId>,
    pub result_slot: Option<ItemId>,
    forge_btn_unlocked: bool,
    filter: Filter,
    hovered_over_item: Option<ItemId>,
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
            hovered_over_item: None,
        }
    }

    pub fn reset_screen(&mut self) {
        self.slot_1 = None;
        self.slot_2 = None;
        self.result_slot = None;
        self.forge_btn_unlocked = false;
        self.filter = Filter::All;
        self.hovered_over_item = None;
    }

    pub fn draw_forge(&mut self, ui: &mut Ui, profile: &mut Profile) {
        heading_area(ui, "Forge", |ui| {
            draw_filter_bar(ui, &mut self.filter);
        });

        let crafting_height = Self::crafting_panel_height(ui);
        let forge_cost = self.forge_cost();
        let info = if let Some(id) = self.result_slot {
            get_item(id)
        } else if let Some(id) = self.slot_1 {
            get_item(id)
        } else {
            self.hovered_over_item.and_then(get_item)
        };

        ui.horizontal_top(|ui| {
            let side_height = ui.available_height().max(0.0);
            let info_height = (side_height - crafting_height - CRAFTING_PANEL_GAP).max(0.0);

            ui.allocate_ui_with_layout(
                egui::vec2(SIDE_PANEL_WIDTH, side_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;

                    item_info_panel(ui, SIDE_PANEL_WIDTH, info_height, info);
                    ui.add_space(CRAFTING_PANEL_GAP);

                    hub_panel(ui, SIDE_PANEL_WIDTH, Some(crafting_height), |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;

                        panel_heading(ui, "Crafting");
                        ui.add_space(CRAFTING_HEADING_GAP);

                        let can_forge = self.can_forge();

                        self.forge_btn_unlocked = can_forge
                            && self.result_slot.is_none()
                            && profile.gold > forge_cost;

                        let btn = egui::Button::new("Forge!")
                            .min_size(egui::vec2(ui.available_width(), CRAFTING_BUTTON_HEIGHT));

                        if ui.add_enabled(self.forge_btn_unlocked, btn).clicked() {
                            profile.gold = profile.gold.saturating_sub(forge_cost);
                            self.forge_items();
                        }

                        ui.add_space(CRAFTING_COST_GAP);
                        let cost_value = if can_forge {
                            forge_cost.to_string()
                        } else {
                            String::from("-")
                        };
                        panel_row(ui, "FORGE COST", &cost_value);

                        ui.add_space(CRAFTING_GRID_GAP);
                        self.forge_crafting(ui, profile);
                    });
                },
            );

            ui.add_space(15.0);

            let grid_width = ui.available_width().max(0.0);
            let grid_height = ui.available_height().max(0.0);
            hub_panel(ui, grid_width, Some(grid_height), |ui| {
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

        let (clicked, hovered) = draw_inventory_grid(
            ui,
            profile.items(),
            visible,
            slot_size,
            num_cols,
            HoverOnSlot::PlaceIntoForge,
        );

        if let Some(id) = clicked {
            let _ = self.push_to_slot(id, profile);
        }

        self.hovered_over_item = hovered;
    }

    fn forge_crafting(&mut self, ui: &mut Ui, profile: &mut Profile) {
        let mut remove_first = false;
        let mut remove_second = false;

        let row_width = 2.0 * CRAFTING_SLOT_SIZE + CRAFTING_SLOT_GAP;
        let side_pad = ((ui.available_width() - row_width) * 0.5).max(0.0);

        ui.horizontal(|ui| {
            ui.add_space(side_pad);

            if let Some(id) = self.slot_1 {
                if inventory_slot(
                    ui,
                    CRAFTING_SLOT_SIZE,
                    item_name(id),
                    HoverOnSlot::RemoveFromForge,
                )
                .clicked()
                {
                    remove_first = true;
                }
            } else {
                inventory_slot(ui, CRAFTING_SLOT_SIZE, "", HoverOnSlot::None);
            }

            ui.add_space(CRAFTING_SLOT_GAP);

            if let Some(id) = self.slot_2 {
                if inventory_slot(
                    ui,
                    CRAFTING_SLOT_SIZE,
                    item_name(id),
                    HoverOnSlot::RemoveFromForge,
                )
                .clicked()
                {
                    remove_second = true;
                }
            } else {
                inventory_slot(ui, CRAFTING_SLOT_SIZE, "", HoverOnSlot::None);
            }
        });

        ui.add_space(CRAFTING_ARROW_GAP);
        let (arrow_rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), CRAFTING_ARROW_HEIGHT),
            egui::Sense::hover(),
        );
        paint_down_arrow(ui.painter(), arrow_rect, CRAFTING_ARROW_COLOR);
        ui.add_space(CRAFTING_ARROW_GAP);

        ui.vertical_centered(|ui| {
            if let Some(id) = self.result_slot {
                if inventory_slot(
                    ui,
                    CRAFTING_SLOT_SIZE,
                    item_name(id),
                    HoverOnSlot::CollectFromForge,
                )
                .clicked()
                {
                    profile.add_item(id);
                    self.result_slot = None;
                }
            } else {
                inventory_slot(ui, CRAFTING_SLOT_SIZE, "Result", HoverOnSlot::None);
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

    fn crafting_panel_height(ui: &Ui) -> f32 {
        let heading_height = ui.text_style_height(&egui::TextStyle::Small);
        let row_height = ui
            .text_style_height(&egui::TextStyle::Body)
            .max(ui.spacing().interact_size.y);
        let button_height = (ui.text_style_height(&egui::TextStyle::Button)
            + 2.0 * ui.spacing().button_padding.y)
            .max(ui.spacing().interact_size.y)
            .max(CRAFTING_BUTTON_HEIGHT);
        let crafting_area_height = 2.0 * CRAFTING_SLOT_SIZE
            + 2.0 * CRAFTING_ARROW_GAP
            + CRAFTING_ARROW_HEIGHT;

        let content_height = heading_height
            + CRAFTING_HEADING_GAP
            + button_height
            + CRAFTING_COST_GAP
            + row_height
            + CRAFTING_GRID_GAP
            + crafting_area_height;

        content_height + panel_chrome_height() + CRAFTING_HEIGHT_SAFETY
    }

    fn can_forge(&self) -> bool {
        let (Some(first), Some(second)) = (self.slot_1, self.slot_2) else {
            return false;
        };

        first == second
            && get_item(first).is_some_and(|item| item.forges_into().is_some())
    }

    fn forge_cost(&self) -> u16 {
        self.slot_1
            .and_then(get_item)
            .map_or(0, |item| ITEM_PRICE_TO_FORGE_COST_FACTOR * item.price())
    }

    fn forge_items(&mut self) {
        let (Some(a), Some(b)) = (self.slot_1, self.slot_2) else {
            return;
        };
        if a != b {
            return;
        }
        let Some(item) = get_item(a) else {
            return;
        };
        let Some(result_id) = item.forges_into() else {
            return;
        };
        self.slot_1 = None;
        self.slot_2 = None;
        self.result_slot = Some(result_id);
    }
}

fn paint_down_arrow(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center_x = rect.center().x;
    let tip = egui::pos2(center_x, rect.bottom() - 2.0);
    let head_height = 8.0;
    let head_width = 10.0;
    let shaft_start = egui::pos2(center_x, rect.top() + 2.0);
    let shaft_end = egui::pos2(center_x, tip.y - head_height);

    painter.line_segment([shaft_start, shaft_end], egui::Stroke::new(2.0, color));
    painter.add(egui::Shape::convex_polygon(
        vec![
            tip,
            egui::pos2(center_x - head_width * 0.5, tip.y - head_height),
            egui::pos2(center_x + head_width * 0.5, tip.y - head_height),
        ],
        color,
        egui::Stroke::NONE,
    ));
}

/// Utility to name crafting slots in forge.
/// Intentionaly returns "" if id doesn't exist.
fn item_name(id: ItemId) -> &'static str {
    get_item(id).map_or("", |i| i.name())
}
