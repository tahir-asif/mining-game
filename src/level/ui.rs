use crate::level::abilities::BOMB_COST;
use crate::level::{levels::Goal, player::Player};
use crate::styles::star_text;

use egui_macroquad::egui::{
    self, Align2, Color32, CornerRadius, CursorIcon, Response, Sense, Stroke, StrokeKind,
    TextStyle, Ui, WidgetInfo, WidgetType,
};

const HUD_MARGIN: f32 = 16.0;
const HUD_HEIGHT: f32 = 56.0;
const HUD_GAP: f32 = 8.0;
const HUD_CORNER_RADIUS: u8 = 6;

const PANEL_FILL: Color32 = Color32::from_rgb(30, 32, 42);
const PANEL_BORDER: Color32 = Color32::from_rgb(72, 78, 100);
const VALUE_TEXT: Color32 = Color32::from_rgb(240, 240, 245);
const MUTED_TEXT: Color32 = Color32::from_rgb(150, 155, 175);

const ENERGY_ACCENT: Color32 = Color32::from_rgb(235, 100, 100);
const GOLD_ACCENT: Color32 = Color32::from_rgb(235, 190, 80);
const GOAL_ACCENT: Color32 = Color32::from_rgb(140, 190, 255);
const STAR_ACCENT: Color32 = Color32::from_rgb(245, 230, 140);
const TECH_ACCENT: Color32 = Color32::from_rgb(100, 220, 210);

const ABILITY_FILL: Color32 = Color32::from_rgb(36, 39, 50);
const ABILITY_BORDER: Color32 = Color32::from_rgb(84, 90, 110);
const ABILITY_HOVER_FILL: Color32 = Color32::from_rgb(52, 57, 74);
const ABILITY_HOVER_BORDER: Color32 = Color32::from_rgb(200, 205, 220);
const ABILITY_PRESSED_FILL: Color32 = Color32::from_rgb(74, 82, 104);
const ABILITY_ACTIVE_FILL: Color32 = Color32::from_rgb(22, 92, 88);
const ABILITY_ACTIVE_HOVER_FILL: Color32 = Color32::from_rgb(30, 116, 110);
const ABILITY_ACTIVE_PRESSED_FILL: Color32 = Color32::from_rgb(18, 75, 72);
const ABILITY_ACTIVE_BORDER: Color32 = Color32::from_rgb(110, 240, 220);
const ABILITY_ACTIVE_COST: Color32 = Color32::from_rgb(190, 245, 235);
const ABILITY_DISABLED_FILL: Color32 = Color32::from_rgb(28, 30, 37);
const ABILITY_DISABLED_BORDER: Color32 = Color32::from_rgb(55, 58, 68);
const ABILITY_DISABLED_TEXT: Color32 = Color32::from_rgb(100, 103, 115);

const ENERGY_WIDTH: f32 = 104.0;
const GOLD_WIDTH: f32 = 104.0;
const GOAL_WIDTH: f32 = 210.0;
const STARS_WIDTH: f32 = 104.0;
const TECH_WIDTH: f32 = 88.0;
const ABILITY_WIDTH: f32 = 112.0;
const AUTO_MINE_WIDTH: f32 = 190.0;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectedAbility {
    #[default]
    None,
    Bomb,
}

impl SelectedAbility {
    pub const ABILITY_BUTTONS: &'static [Self] = &[Self::Bomb];

    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Bomb => "BOMB",
        }
    }

    pub const fn cost(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Bomb => BOMB_COST,
        }
    }
}

pub enum HudAction {
    None,
    ToggleAbility(SelectedAbility),
    ToggleAutoMine,
    Deselect,
}

struct AbilityRow {
    clicked: Option<SelectedAbility>,
    consumed_click: bool,
}

pub fn draw_ui(
    player: &Player,
    goal: Goal,
    gold_mined: u16,
    active: SelectedAbility,
    auto_mine: bool,
) -> HudAction {
    let mut action = HudAction::None;

    egui_macroquad::ui(|ctx| {
        egui::Area::new("hud_top_left".into())
            .anchor(Align2::LEFT_TOP, egui::vec2(HUD_MARGIN, HUD_MARGIN))
            .show(ctx, |ui| {
                hud_top_left(ui, player);
            });

        egui::Area::new("hud_top_right".into())
            .anchor(Align2::RIGHT_TOP, egui::vec2(-HUD_MARGIN, HUD_MARGIN))
            .show(ctx, |ui| {
                hud_top_right(ui, player, goal, gold_mined);
            });

        let auto_mine_clicked = egui::Area::new("hud_bottom_left".into())
            .anchor(Align2::LEFT_BOTTOM, egui::vec2(HUD_MARGIN, -HUD_MARGIN))
            .show(ctx, |ui| hud_bottom_left(ui, auto_mine))
            .inner;

        let ability_row = egui::Area::new("hud_bottom_right".into())
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-HUD_MARGIN, -HUD_MARGIN))
            .show(ctx, |ui| hud_bottom_right(ui, player, active))
            .inner;

        action = if let Some(clicked) = ability_row.clicked {
            HudAction::ToggleAbility(clicked)
        } else if auto_mine_clicked {
            HudAction::ToggleAutoMine
        } else if !ability_row.consumed_click && ctx.input(|input| input.pointer.primary_clicked())
        {
            HudAction::Deselect
        } else {
            HudAction::None
        };
    });

    action
}

fn hud_bottom_left(ui: &mut Ui, auto_mine: bool) -> bool {
    toggle_button(
        ui,
        AUTO_MINE_WIDTH,
        "TOGGLE MINING",
        if auto_mine { "ON" } else { "OFF" },
        auto_mine,
        true,
    )
    .clicked()
}

fn hud_top_left(ui: &mut Ui, player: &Player) {
    stat_card(
        ui,
        ENERGY_WIDTH,
        "ENERGY",
        &player.energy.to_string(),
        ENERGY_ACCENT,
    );
}

fn hud_top_right(ui: &mut Ui, player: &Player, goal: Goal, gold_mined: u16) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = HUD_GAP;

        stat_card(
            ui,
            GOLD_WIDTH,
            "GOLD",
            &player.collected_gold.to_string(),
            GOLD_ACCENT,
        );
        stat_card(
            ui,
            GOAL_WIDTH,
            "GOAL",
            &goal.to_str(gold_mined),
            GOAL_ACCENT,
        );
        stat_card(
            ui,
            STARS_WIDTH,
            "STARS",
            star_text(player.stars),
            STAR_ACCENT,
        );
    });
}

fn hud_bottom_right(ui: &mut Ui, player: &Player, active: SelectedAbility) -> AbilityRow {
    let mut clicked = None;
    let mut consumed_click = false;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = HUD_GAP;

        stat_card(
            ui,
            TECH_WIDTH,
            "TECH",
            &player.tech.to_string(),
            TECH_ACCENT,
        );

        for &ability in SelectedAbility::ABILITY_BUTTONS {
            let is_active = ability == active;
            let enabled = is_active || player.tech >= ability.cost();
            let response = ability_button(ui, ability, is_active, enabled);

            if response.clicked() {
                consumed_click = true;
                if enabled {
                    clicked = Some(ability);
                }
            }
        }
    });

    AbilityRow {
        clicked,
        consumed_click,
    }
}

fn paint_panel(ui: &Ui, rect: egui::Rect, fill: Color32, border: Color32, border_width: f32) {
    let painter = ui.painter();
    let corner_radius = CornerRadius::same(HUD_CORNER_RADIUS);

    painter.rect_filled(rect, corner_radius, fill);
    painter.rect_stroke(
        rect,
        corner_radius,
        Stroke::new(border_width, border),
        StrokeKind::Inside,
    );
}

fn stat_card(ui: &mut Ui, width: f32, title: &str, value: &str, accent: Color32) -> Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, HUD_HEIGHT), Sense::hover());
    paint_panel(ui, rect, PANEL_FILL, PANEL_BORDER, 1.0);

    let title_font = TextStyle::Small.resolve(ui.style());
    let value_font = TextStyle::Body.resolve(ui.style());
    let painter = ui.painter();

    painter.text(
        egui::pos2(rect.center().x, rect.top() + 15.0),
        Align2::CENTER_CENTER,
        title,
        title_font,
        accent,
    );
    painter.text(
        egui::pos2(rect.center().x, rect.top() + 38.0),
        Align2::CENTER_CENTER,
        value,
        value_font,
        VALUE_TEXT,
    );

    response
}

fn ability_button(ui: &mut Ui, ability: SelectedAbility, active: bool, enabled: bool) -> Response {
    toggle_button(
        ui,
        ABILITY_WIDTH,
        ability.name(),
        &format!("{} TECH", ability.cost()),
        active,
        enabled,
    )
}

fn toggle_button(
    ui: &mut Ui,
    width: f32,
    label: &str,
    subtitle: &str,
    active: bool,
    enabled: bool,
) -> Response {
    let (rect, mut response) =
        ui.allocate_exact_size(egui::vec2(width, HUD_HEIGHT), Sense::click());

    if enabled {
        response = response.on_hover_cursor(CursorIcon::PointingHand);
    }

    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));

    let hovered = enabled && response.hovered();
    let pressed = enabled && response.is_pointer_button_down_on();

    let (fill, border, label_color, subtitle_color) = if !enabled {
        (
            ABILITY_DISABLED_FILL,
            ABILITY_DISABLED_BORDER,
            ABILITY_DISABLED_TEXT,
            ABILITY_DISABLED_TEXT,
        )
    } else if active {
        let fill = if pressed {
            ABILITY_ACTIVE_PRESSED_FILL
        } else if hovered {
            ABILITY_ACTIVE_HOVER_FILL
        } else {
            ABILITY_ACTIVE_FILL
        };
        (fill, ABILITY_ACTIVE_BORDER, VALUE_TEXT, ABILITY_ACTIVE_COST)
    } else if pressed {
        (ABILITY_PRESSED_FILL, Color32::WHITE, VALUE_TEXT, MUTED_TEXT)
    } else if hovered {
        (
            ABILITY_HOVER_FILL,
            ABILITY_HOVER_BORDER,
            VALUE_TEXT,
            MUTED_TEXT,
        )
    } else {
        (ABILITY_FILL, ABILITY_BORDER, VALUE_TEXT, MUTED_TEXT)
    };

    let border_width = if enabled && (active || hovered || pressed) {
        2.0
    } else {
        1.0
    };
    paint_panel(ui, rect, fill, border, border_width);

    let label_font = TextStyle::Body.resolve(ui.style());
    let subtitle_font = TextStyle::Small.resolve(ui.style());
    let painter = ui.painter();

    painter.text(
        egui::pos2(rect.center().x, rect.top() + 16.0),
        Align2::CENTER_CENTER,
        label,
        label_font,
        label_color,
    );
    painter.text(
        egui::pos2(rect.center().x, rect.top() + 39.0),
        Align2::CENTER_CENTER,
        subtitle,
        subtitle_font,
        subtitle_color,
    );

    response
}
