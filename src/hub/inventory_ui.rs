use egui_macroquad::egui::{self, Align, Color32, CornerRadius, Layout, Stroke, Ui};

use crate::{
    hub::{
        Filter, GRID_SPACING, TARGET_GRID_SIZE, TOOLBAR_HEIGHT,
        slot::{HoverOnSlot, inventory_slot},
    },
    styles::{header_text, normal_text, small_text},
};
use shared::{
    ItemKind, ItemRef,
    items::{ItemId, get_item},
};

pub const SIDE_PANEL_WIDTH: f32 = 250.0;

const HEADER_CONTROLS_HEIGHT: f32 = 42.0;
const PANEL_MARGIN: f32 = 12.0;
const PANEL_STROKE_WIDTH: f32 = 1.0;
const PANEL_CORNER_RADIUS: u8 = 6;
const PANEL_FILL: Color32 = Color32::from_rgb(30, 32, 42);
const PANEL_BORDER: Color32 = Color32::from_rgb(72, 78, 100);
const INFO_ACCENT: Color32 = Color32::from_rgb(140, 190, 255);
const INFO_MUTED: Color32 = Color32::from_rgb(150, 155, 175);

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
) -> (Option<ItemId>, Option<ItemId>) {
    let mut clicked = None;
    let mut hovered = None;

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

                        let slot = inventory_slot(ui, slot_size, name, hover);
                        if slot.clicked() {
                            clicked = Some(id);
                        }
                        if slot.hovered() {
                            hovered = Some(id);
                        }

                        visible_count += 1;
                        if visible_count % num_cols == 0 {
                            ui.end_row();
                        }
                    }
                });
        });

    (clicked, hovered)
}

pub fn panel_chrome_height() -> f32 {
    (PANEL_MARGIN + PANEL_STROKE_WIDTH) * 2.0
}

pub fn hub_panel(ui: &mut Ui, width: f32, height: Option<f32>, add_contents: impl FnOnce(&mut Ui)) {
    let chrome = panel_chrome_height();
    let inner_width = (width - chrome).max(0.0);

    ui.scope_builder(
        egui::UiBuilder::new().layout(Layout::top_down(Align::Min)),
        |ui| {
            egui::Frame::new()
                .fill(PANEL_FILL)
                .stroke(Stroke::new(PANEL_STROKE_WIDTH, PANEL_BORDER))
                .corner_radius(CornerRadius::same(PANEL_CORNER_RADIUS))
                .inner_margin(PANEL_MARGIN)
                .show(ui, |ui| {
                    ui.set_width(inner_width);
                    if let Some(height) = height {
                        ui.set_min_height((height - chrome).max(0.0));
                    }
                    add_contents(ui);
                });
        },
    );
}

pub fn heading_area(ui: &mut Ui, title: &str, add_controls: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), TOOLBAR_HEIGHT),
        Layout::top_down(Align::LEFT),
        |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.label(header_text(title));
            ui.add_space(10.0);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), HEADER_CONTROLS_HEIGHT),
                Layout::left_to_right(Align::Center),
                add_controls,
            );
        },
    );
}

pub fn item_info_panel(ui: &mut Ui, width: f32, height: f32, info: Option<ItemRef>) {
    hub_panel(ui, width, Some(height), |ui| {
        let Some(item) = info else {
            ui.label(normal_text("Hover over an item to see its info").color(INFO_MUTED));
            return;
        };

        ui.label(normal_text(item.name()));
        ui.add_space(8.0);
        panel_row(ui, "SELL PRICE", &item.price().to_string());
        panel_row(ui, stat_label(item), &item.ability().to_string());
    });
}

pub fn panel_heading(ui: &mut Ui, text: &str) {
    ui.label(normal_text(text));
}

pub fn panel_row(ui: &mut Ui, title: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(small_text(title).color(INFO_ACCENT));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(normal_text(value));
        });
    });
}

fn stat_label(item: ItemRef) -> &'static str {
    match item.kind() {
        ItemKind::Backpack => "STARTING TECH",
        ItemKind::Hat => "STARTING ENERGY",
        ItemKind::Pickaxe => "MINING POWER",
    }
}
