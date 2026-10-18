use crate::{
    game::{GameState, TransitionState},
    level::levels::LEVELS,
    save::Profile,
    styles::{header_text, small_text},
};
use egui_macroquad::egui::{self, Align, Color32, Layout, Ui};

enum CenterPanelState {
    Inventory,
    Crafting,
}

pub struct Hub {
    center_panel_state: CenterPanelState,
}

impl Hub {
    pub const fn new() -> Self {
        Hub {
            center_panel_state: CenterPanelState::Inventory,
        }
    }

    pub fn hub_update(&mut self, profile: &mut Profile) -> TransitionState {
        let mut transition = TransitionState::None;
        let fill_colour = Color32::from_rgb(95, 0, 223);

        egui_macroquad::ui(|ctx| {
            let frame = egui::Frame::new().fill(fill_colour).inner_margin(10.0);

            let panel_width = ctx.screen_rect().width() * 0.20;
            egui::SidePanel::left("SideBar")
                .frame(frame.fill(fill_colour.linear_multiply(0.1)))
                .exact_width(panel_width)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_left_sidebar(ui, profile, &mut transition)
                });

            let panel_width = ctx.screen_rect().width() * 0.10;
            egui::SidePanel::right("Levels")
                .frame(frame.fill(fill_colour.linear_multiply(0.1)))
                .exact_width(panel_width)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_right_sidebar(ui, &mut transition);
                });

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                match self.center_panel_state {
                    CenterPanelState::Inventory => draw_inventory(ui, profile),
                    CenterPanelState::Crafting => draw_crafting(ui),
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
                ui.label(small_text("Gold:"));

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let label = format!("{}", profile.gold);
                    ui.label(small_text(&label));
                });
            });

            ui.add_space(20.0);
            if ui.button("Inventory").clicked() {
                self.center_panel_state = CenterPanelState::Inventory;
            }
            if ui.button("Crafting").clicked() {
                self.center_panel_state = CenterPanelState::Crafting;
            }
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label("Equipped");
                let label = format!("> {}", profile.equipped.pickaxe.name);
                ui.label(small_text(&label));
                let label = format!("> {}", profile.equipped.hat.name);
                ui.label(small_text(&label));
                let label = format!("> {}", profile.equipped.backpack.name);
                ui.label(small_text(&label));
            });

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if ui.button("Main Menu").clicked() {
                    *transition = TransitionState::GoTo(GameState::MainMenu);
                }
            });
        });
    }

    fn draw_right_sidebar(&mut self, ui: &mut Ui, transition: &mut TransitionState) {
        ui.vertical_centered_justified(|ui| {
            ui.label("Levels");
            for level in LEVELS {
                let label = format!("{}", level.id + 1);
                if ui.button(&label).clicked() {
                    *transition = TransitionState::StartLevel(level.id);
                }
            }
        });
    }
}

fn draw_inventory(ui: &mut Ui, profile: &mut Profile) {
    ui.vertical(|ui| {
        ui.label(header_text("Inventory"));

        let grid_spacing = 8.0;
        let column_target_width = 120.0;
        let available_width = ui.available_width();
        let num_cols = ((available_width + grid_spacing) / (column_target_width + grid_spacing))
            .floor()
            .max(1.0) as usize;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("inventory-grid")
                .spacing(egui::vec2(grid_spacing, grid_spacing))
                .show(ui, |ui| {
                    for (i, item) in profile.items.iter().enumerate() {
                        ui.button(item.name());

                        if (i + 1) % num_cols == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    });
}

fn draw_crafting(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.label(header_text("Crafting"));
    });
}
