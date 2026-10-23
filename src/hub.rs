mod forge;
mod inventory;
mod inventory_ui;
mod slot;

use crate::{
    game::{GameState, TransitionState},
    hub::{forge::Forge, inventory::Inventory},
    level::levels::LEVELS,
    profile::Profile,
    styles::{normal_text, small_text},
};
use egui_macroquad::egui::{self, Align, Color32, Layout, Ui};
use shared::{
    ItemId,
    items::{BACKPACK_RANGE, HAT_RANGE, PICKAXE_RANGE},
};

const GRID_SPACING: f32 = 8.0;
const TARGET_GRID_SIZE: f32 = 120.0;
const TOOLBAR_HEIGHT: f32 = 100.0;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Filter {
    All,
    Pickaxes,
    Hats,
    Backpacks,
}

impl Filter {
    pub fn matches(self, id: ItemId) -> bool {
        match self {
            Filter::All => true,
            Filter::Pickaxes => PICKAXE_RANGE.contains(&id),
            Filter::Hats => HAT_RANGE.contains(&id),
            Filter::Backpacks => BACKPACK_RANGE.contains(&id),
        }
    }
}

enum CenterPanelState {
    Inventory,
    Forge,
}

pub struct Hub {
    center_panel_state: CenterPanelState,
    forge: Forge,
    inventory: Inventory,
}

impl Hub {
    pub const fn new() -> Self {
        Hub {
            center_panel_state: CenterPanelState::Inventory,
            forge: Forge::new(),
            inventory: Inventory::new(),
        }
    }

    pub fn hub_update(&mut self, profile: &mut Profile) -> TransitionState {
        let mut transition = TransitionState::None;
        let fill_colour = Color32::from_rgb(95, 0, 223);

        egui_macroquad::ui(|ctx| {
            let frame = egui::Frame::new().fill(fill_colour).inner_margin(15.0);

            let panel_width = 200.0;
            egui::SidePanel::left("SideBar")
                .frame(frame.fill(fill_colour.linear_multiply(0.1)))
                .exact_width(panel_width)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_left_sidebar(ui, profile, &mut transition)
                });

            let panel_width = 100.0;
            egui::SidePanel::right("Levels")
                .frame(frame.fill(fill_colour.linear_multiply(0.1)))
                .exact_width(panel_width)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_right_sidebar(ui, &mut transition, profile);
                });

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                match self.center_panel_state {
                    CenterPanelState::Inventory => {
                        self.forge.reset_screen();
                        self.inventory.draw(ui, profile);
                    }
                    CenterPanelState::Forge => {
                        self.inventory.reset_screen();
                        self.forge.draw_forge(ui, profile);
                    }
                }
            });
        });

        transition
    }

    fn draw_left_sidebar(
        &mut self,
        ui: &mut Ui,
        profile: &mut Profile,
        transition: &mut TransitionState,
    ) {
        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                ui.label("Gold:");

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let label = format!("{}", profile.gold);
                    ui.label(normal_text(&label).color(egui::Color32::GOLD));
                });
            });

            ui.add_space(40.0);
            if ui.button("Inventory").clicked() {
                self.center_panel_state = CenterPanelState::Inventory;
            }
            ui.add_space(15.0);
            if ui.button("Forge").clicked() {
                self.center_panel_state = CenterPanelState::Forge;
            }
            ui.add_space(40.0);
            ui.vertical(|ui| {
                ui.label("Equipped");
                ui.add_space(10.0);

                let label = format!("> {}", profile.equipped.pickaxe.name);
                ui.label(small_text(&label));
                let label = format!("Mining Power: {}", profile.equipped.pickaxe.mining_power);
                ui.label(small_text(&label));
                ui.add_space(10.0);

                let label = format!("> {}", profile.equipped.hat.name);
                ui.label(small_text(&label));
                let label = format!("Starting Energy: {}", profile.equipped.hat.starting_energy);
                ui.label(small_text(&label));
                ui.add_space(10.0);

                let label = format!("> {}", profile.equipped.backpack.name);
                ui.label(small_text(&label));
                let label = format!("Starting Tech: {}", profile.equipped.backpack.starting_tech);
                ui.label(small_text(&label));
            });

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if ui.button("Main Menu").clicked() {
                    self.forge.return_all(profile);
                    *transition = TransitionState::GoTo(GameState::MainMenu);
                }
            });
        });
    }

    fn draw_right_sidebar(
        &mut self,
        ui: &mut Ui,
        transition: &mut TransitionState,
        profile: &mut Profile,
    ) {
        ui.vertical_centered_justified(|ui| {
            ui.label("Levels");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut unlocked = true;
                for level in LEVELS {
                    let label = format!("{}", level.id + 1);
                    let btn = egui::Button::new(&label);

                    if level.id > profile.latest_completed_level {
                        unlocked = false;
                    }

                    if ui.add_enabled(unlocked, btn).clicked() {
                        self.forge.return_all(profile);
                        *transition = TransitionState::StartLevel(level.id);
                    }
                }
            });
        });
    }
}
