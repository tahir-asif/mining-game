#![recursion_limit = "256"]

mod palette;

use eframe::egui;
use palette::{Brush, all_brushes, color, color_for_id, label, name_for_id};
use shared::block::{BlockCategory, BlockType, ID_EMPTY, blocktype_from_id};
use shared::items::{STARTING_PICKAXE_ID, get_item};
use shared::{ITEMS, ItemId, RawGoal, RawLevel};
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Paint,
    Erase,
    Rect,
    Line,
    Fill,
}

const SAVED_VISIBLE: Duration = Duration::from_secs(2);
const DELETE_CONFIRM_TIMEOUT: Duration = Duration::from_secs(2);
const HISTORY_MAX: usize = 50;

struct History {
    undo: Vec<Vec<RawLevel>>,
    redo: Vec<Vec<RawLevel>>,
}

impl History {
    fn new() -> Self {
        Self {
            undo: vec![],
            redo: vec![],
        }
    }

    fn snapshot(&mut self, state: &[RawLevel]) {
        self.undo.push(state.to_vec());
        if self.undo.len() > HISTORY_MAX {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    fn undo(&mut self, current: &mut Vec<RawLevel>) -> bool {
        match self.undo.pop() {
            Some(prev) => {
                self.redo.push(std::mem::replace(current, prev));
                true
            }
            None => false,
        }
    }

    fn redo(&mut self, current: &mut Vec<RawLevel>) -> bool {
        match self.redo.pop() {
            Some(next) => {
                self.undo.push(std::mem::replace(current, next));
                true
            }
            None => false,
        }
    }
}

fn put_cell(level: &mut RawLevel, x: u8, z: u8, id: u8) {
    if x >= level.width || z >= level.height {
        return;
    }
    if id != ID_EMPTY && (x, z) == level.spawn {
        return;
    }
    level.grid[z as usize][x as usize] = id;
}

fn is_goal_id(id: u8) -> bool {
    blocktype_from_id(id).is_some_and(|kind| kind.is_goal())
}

fn goal_cell(level: &RawLevel) -> Option<(u8, u8)> {
    match level.goal {
        RawGoal::Collect { x, z } => Some((x, z)),
        RawGoal::MineAllGold(_) => None,
    }
}

fn clear_goal_chests(level: &mut RawLevel) {
    for z in 0..level.height {
        for x in 0..level.width {
            let (xi, zi) = (x as usize, z as usize);
            if is_goal_id(level.grid[zi][xi]) {
                level.grid[zi][xi] = ID_EMPTY;
                level.chests.retain(|(c, _)| *c != (x, z));
            }
        }
    }
}

fn find_goal_cell(level: &RawLevel) -> Option<(u8, u8)> {
    for z in 0..level.height {
        for x in 0..level.width {
            if (x, z) != level.spawn {
                return Some((x, z));
            }
        }
    }
    None
}

fn count_gold(level: &RawLevel) -> u16 {
    level
        .grid
        .iter()
        .flatten()
        .filter(|&&id| {
            blocktype_from_id(id).is_some_and(|kind| kind.category() == BlockCategory::GoldOre)
        })
        .count() as u16
}

fn sync_gold_count(level: &mut RawLevel) {
    let count = count_gold(level);
    if let RawGoal::MineAllGold(gold_blocks) = &mut level.goal {
        *gold_blocks = count;
    }
}

fn normalize_goal_chest(level: &mut RawLevel) {
    let RawGoal::Collect { x, z } = level.goal else {
        clear_goal_chests(level);
        return;
    };
    let mut gx = x.min(level.width.saturating_sub(1));
    let mut gz = z.min(level.height.saturating_sub(1));
    if (gx, gz) == level.spawn {
        if let Some((fx, fz)) = find_goal_cell(level) {
            gx = fx;
            gz = fz;
        } else {
            return;
        }
    }
    let item = level
        .chests
        .iter()
        .find(|((cx, cz), _)| (*cx, *cz) == (gx, gz))
        .map(|(_, item)| *item)
        .unwrap_or(STARTING_PICKAXE_ID);
    clear_goal_chests(level);
    level.goal = RawGoal::Collect { x: gx, z: gz };
    level.grid[gz as usize][gx as usize] = BlockType::GoalChest.id();
    level.chests.retain(|(c, _)| *c != (gx, gz));
    level.chests.push(((gx, gz), item));
}

pub struct LevelEditor {
    levels: Vec<RawLevel>,
    levels_path: PathBuf,
    current: usize,
    tool: Tool,
    selected_brush: Brush,
    hover_cell: Option<(u8, u8)>,
    dirty: bool,
    goal_pick_mode: bool,
    pending_recenter: bool,

    // Drag state for Rect/Line
    drag_start_cell: Option<(u8, u8)>,
    drag_current_cell: Option<(u8, u8)>,
    drag_is_active: bool,

    // Pan
    pan: egui::Vec2,
    pan_drag_origin: Option<(egui::Pos2, egui::Vec2)>,

    // Chest flow
    pending_chest: Option<(u8, u8)>,
    pending_chest_kind: BlockType,
    pending_chest_item: ItemId,

    // Status / save / delete
    saved_at: Option<Instant>,
    pending_delete_at: Option<Instant>,

    history: History,
}

impl LevelEditor {
    pub fn new(levels_path: PathBuf) -> Self {
        let mut levels: Vec<RawLevel> = std::fs::read_to_string(&levels_path)
            .ok()
            .and_then(|s: String| ron::from_str::<Vec<RawLevel>>(&s).ok())
            .unwrap_or_default();

        for level in levels.iter_mut() {
            let (sx, sz) = (level.spawn.0 as usize, level.spawn.1 as usize);
            if sz < level.grid.len() && sx < level.grid[sz].len() {
                level.grid[sz][sx] = ID_EMPTY;
            }
        }

        Self {
            levels,
            levels_path,
            current: 0,
            tool: Tool::Paint,
            selected_brush: Brush::Empty,
            hover_cell: None,
            dirty: false,
            goal_pick_mode: false,
            pending_recenter: true,
            drag_start_cell: None,
            drag_current_cell: None,
            drag_is_active: false,
            pan: egui::Vec2::ZERO,
            pan_drag_origin: None,
            pending_chest: None,
            pending_chest_kind: BlockType::ItemChest,
            pending_chest_item: STARTING_PICKAXE_ID,
            saved_at: None,
            pending_delete_at: None,
            history: History::new(),
        }
    }

    fn save(&mut self) {
        for level in &mut self.levels {
            sync_gold_count(level);
            normalize_goal_chest(level);
        }
        let config = ron::ser::PrettyConfig::new()
            .separate_tuple_members(false) // Collect(x: 3, z: 3) on one line
            .depth_limit(3);
        match ron::ser::to_string_pretty(&self.levels, config) {
            Ok(s) => match std::fs::write(&self.levels_path, s) {
                Ok(_) => {
                    self.dirty = false;
                    self.saved_at = Some(Instant::now());
                }
                Err(e) => {
                    self.saved_at = None;
                    eprintln!("save error: {e}");
                }
            },
            Err(e) => {
                self.saved_at = None;
                eprintln!("serialize error: {e}");
            }
        }
    }

    fn create_level(&mut self, at: usize) {
        self.history.snapshot(&self.levels);
        let (w, h) = (20u8, 15u8);
        let mut grid = vec![vec![ID_EMPTY; w as usize]; h as usize];
        grid[2][2] = BlockType::GoalChest.id();
        let level = RawLevel {
            spawn: (1, 1),
            width: w,
            height: h,
            goal: RawGoal::Collect { x: 2, z: 2 },
            grid,
            chests: vec![((2, 2), STARTING_PICKAXE_ID)],
        };
        self.levels.insert(at, level);
        self.current = at;
        self.dirty = true;
        self.pan = egui::Vec2::ZERO;
        self.pending_recenter = true;
        self.pending_delete_at = None;
        self.goal_pick_mode = false;
        self.pending_chest = None;
    }

    fn new_level(&mut self) {
        self.create_level(self.levels.len());
    }

    fn insert_level_after_current(&mut self) {
        let at = (self.current + 1).min(self.levels.len());
        self.create_level(at);
    }

    fn insert_level_before_current(&mut self) {
        self.create_level(self.current);
    }

    fn previous_level(&mut self) {
        if self.current > 0 {
            self.current -= 1;
            self.pending_recenter = true;
            self.pending_delete_at = None;
            self.pending_chest = None;
            self.goal_pick_mode = false;
        }
    }

    fn next_level(&mut self) {
        if self.current + 1 < self.levels.len() {
            self.current += 1;
            self.pending_recenter = true;
            self.pending_delete_at = None;
            self.pending_chest = None;
            self.goal_pick_mode = false;
        }
    }

    fn resize_level(&mut self, new_w: u8, new_h: u8) {
        if self.levels.is_empty() {
            return;
        }
        let level = &mut self.levels[self.current];
        let old_w = level.width as usize;
        let old_h = level.height as usize;
        let goal_item = match level.goal {
            RawGoal::Collect { x, z }
                if usize::from(x) < old_w
                    && usize::from(z) < old_h
                    && is_goal_id(level.grid[z as usize][x as usize]) =>
            {
                level
                    .chests
                    .iter()
                    .find(|((cx, cz), _)| (*cx, *cz) == (x, z))
                    .map(|(_, item)| *item)
            }
            _ => None,
        };
        let nw = new_w as usize;
        let nh = new_h as usize;
        let mut new_grid = vec![vec![ID_EMPTY; nw]; nh];
        for z in 0..old_h.min(nh) {
            for x in 0..old_w.min(nw) {
                new_grid[z][x] = level.grid[z][x];
            }
        }
        level.grid = new_grid;
        level.width = new_w;
        level.height = new_h;
        level.spawn.0 = level.spawn.0.min(new_w.saturating_sub(1));
        level.spawn.1 = level.spawn.1.min(new_h.saturating_sub(1));
        level.grid[level.spawn.1 as usize][level.spawn.0 as usize] = ID_EMPTY;
        level.chests.retain(|((x, z), _)| *x < new_w && *z < new_h);
        clear_goal_chests(level);
        if let RawGoal::Collect { x, z } = level.goal {
            let mut gx = x.min(new_w.saturating_sub(1));
            let mut gz = z.min(new_h.saturating_sub(1));
            if (gx, gz) == level.spawn {
                if let Some((fx, fz)) = find_goal_cell(level) {
                    gx = fx;
                    gz = fz;
                }
            }
            level.goal = RawGoal::Collect { x: gx, z: gz };
            if (gx, gz) != level.spawn {
                level.grid[gz as usize][gx as usize] = BlockType::GoalChest.id();
                level.chests.retain(|(c, _)| *c != (gx, gz));
                level
                    .chests
                    .push(((gx, gz), goal_item.unwrap_or(STARTING_PICKAXE_ID)));
            }
        }
        self.dirty = true;
    }

    fn set_goal_chest(&mut self, x: u8, z: u8) {
        if self.levels.is_empty() {
            return;
        }
        let level = &mut self.levels[self.current];
        if (x, z) == level.spawn || x >= level.width || z >= level.height {
            return;
        }
        clear_goal_chests(level);
        level.goal = RawGoal::Collect { x, z };
        level.grid[z as usize][x as usize] = BlockType::GoalChest.id();
        level.chests.retain(|(c, _)| *c != (x, z));
        level.chests.push(((x, z), STARTING_PICKAXE_ID));
        self.pending_chest = Some((x, z));
        self.pending_chest_kind = BlockType::GoalChest;
        self.pending_chest_item = STARTING_PICKAXE_ID;
        self.goal_pick_mode = false;
        self.dirty = true;
    }

    fn delete_level(&mut self) {
        if self.levels.len() <= 1 {
            return;
        }
        self.history.snapshot(&self.levels);
        self.levels.remove(self.current);
        self.current = self.current.min(self.levels.len() - 1);
        self.dirty = true;
        self.pending_delete_at = None;
        self.pending_chest = None;
        self.goal_pick_mode = false;
    }

    fn cell_color(&self, level: &RawLevel, x: u8, z: u8) -> egui::Color32 {
        if (x, z) == level.spawn {
            return color(Brush::Spawn);
        }
        color_for_id(level.grid[z as usize][x as usize])
    }

    /// Apply the current palette entry at a cell. Does not touch history.
    fn apply_cell(&mut self, x: u8, z: u8) {
        if self.levels.is_empty() {
            return;
        }
        let brush = self.selected_brush;
        let level = &mut self.levels[self.current];
        let (xi, zi) = (x as usize, z as usize);

        if goal_cell(level) == Some((x, z)) {
            return;
        }

        if self.tool == Tool::Erase || brush == Brush::Empty {
            level.grid[zi][xi] = ID_EMPTY;
            level.chests.retain(|(c, _)| *c != (x, z));
            self.dirty = true;
            return;
        }

        if matches!(brush, Brush::Block(_)) && (x, z) == level.spawn {
            return;
        }

        match brush {
            Brush::Spawn => {
                level.spawn = (x, z);
                level.grid[zi][xi] = ID_EMPTY;
                level.chests.retain(|(c, _)| *c != (x, z));
                self.dirty = true;
            }
            Brush::Block(BlockType::ItemChest) => {
                self.pending_chest_item = level
                    .chests
                    .iter()
                    .find(|(c, _)| *c == (x, z))
                    .map(|(_, id)| *id)
                    .unwrap_or(STARTING_PICKAXE_ID);
                self.pending_chest_kind = BlockType::ItemChest;
                self.pending_chest = Some((x, z));
            }
            Brush::Block(kind) => {
                put_cell(level, x, z, kind.id());
                level.chests.retain(|(c, _)| *c != (x, z));
                self.dirty = true;
            }
            Brush::Empty => {}
        }
    }

    /// Apply paint to every cell in a rectangle (inclusive).
    fn apply_rect(&mut self, a: (u8, u8), b: (u8, u8)) {
        let (x0, x1) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
        let (z0, z1) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
        for z in z0..=z1 {
            for x in x0..=x1 {
                self.apply_cell(x, z);
            }
        }
    }

    /// Bresenham line between two cells, inclusive.
    fn apply_line(&mut self, a: (u8, u8), b: (u8, u8)) {
        let (mut x0, mut z0) = (a.0 as i32, a.1 as i32);
        let (x1, z1) = (b.0 as i32, b.1 as i32);
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dz = -(z1 - z0).abs();
        let sz = if z0 < z1 { 1 } else { -1 };
        let mut err = dx + dz;
        loop {
            self.apply_cell(x0 as u8, z0 as u8);
            if x0 == x1 && z0 == z1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dz {
                err += dz;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                z0 += sz;
            }
        }
    }

    /// Flood-fill contiguous cells with the same underlying block ID.
    fn apply_fill(&mut self, x: u8, z: u8) {
        if self.levels.is_empty() {
            return;
        }
        let new_id = match self.selected_brush {
            Brush::Spawn => return,
            Brush::Block(kind) if kind.is_item_chest() => return,
            brush => brush.block_id().unwrap_or(ID_EMPTY),
        };

        let level = &mut self.levels[self.current];
        let target_id = level.grid[z as usize][x as usize];
        if new_id == target_id {
            return;
        }

        let (w, h) = (level.width as i32, level.height as i32);
        let mut stack = vec![(x as i32, z as i32)];
        let mut visited = vec![vec![false; w as usize]; h as usize];
        while let Some((cx, cz)) = stack.pop() {
            if cx < 0 || cz < 0 || cx >= w || cz >= h {
                continue;
            }
            let (xi, zi) = (cx as usize, cz as usize);
            if visited[zi][xi] {
                continue;
            }
            visited[zi][xi] = true;
            if level.grid[zi][xi] != target_id {
                continue;
            }
            if is_goal_id(level.grid[zi][xi]) {
                continue;
            }
            put_cell(level, cx as u8, cz as u8, new_id);
            level.chests.retain(|(c, _)| *c != (cx as u8, cz as u8));
            stack.push((cx + 1, cz));
            stack.push((cx - 1, cz));
            stack.push((cx, cz + 1));
            stack.push((cx, cz - 1));
        }
        self.dirty = true;
    }

    fn show_palette(&mut self, ui: &mut egui::Ui) {
        ui.heading("Blocks");
        for brush in all_brushes() {
            let selected = self.selected_brush == brush;
            let (rect, response) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 22.0), egui::Sense::click());
            let painter = ui.painter();
            let (bg, text_color) = if selected {
                (egui::Color32::from_rgb(60, 110, 190), egui::Color32::WHITE)
            } else if response.hovered() {
                (egui::Color32::from_gray(60), egui::Color32::from_gray(230))
            } else {
                (ui.visuals().extreme_bg_color, ui.visuals().text_color())
            };
            painter.rect_filled(rect, 3.0, bg);
            let swatch =
                egui::Rect::from_min_size(rect.min + egui::vec2(4.0, 4.0), egui::vec2(14.0, 14.0));
            painter.rect_filled(swatch, 2.0, color(brush));
            painter.rect_stroke(
                swatch,
                2.0,
                egui::Stroke::new(1.0, egui::Color32::from_gray(90)),
                egui::StrokeKind::Inside,
            );
            painter.text(
                rect.min + egui::vec2(24.0, 11.0),
                egui::Align2::LEFT_CENTER,
                label(brush),
                egui::FontId::proportional(14.0),
                text_color,
            );
            if response.clicked() {
                self.selected_brush = brush;
            }
        }
    }

    fn show_metadata(&mut self, ui: &mut egui::Ui) {
        if self.levels.is_empty() {
            ui.label("No levels yet.");
            return;
        }

        // --- Level ---
        ui.heading("Level");

        let (w, h, spawn) = {
            let l = &self.levels[self.current];
            (l.width, l.height, l.spawn)
        };
        ui.label(format!("Spawn: ({}, {})", spawn.0, spawn.1));

        // Resizable size
        let mut new_w = w;
        let mut new_h = h;
        ui.horizontal(|ui| {
            ui.label("W:");
            if ui.add_enabled(w > 1, egui::Button::new("−")).clicked() {
                new_w = w - 1;
            }
            ui.label(format!("{w:>2}"));
            if ui.add_enabled(w < 255, egui::Button::new("+")).clicked() {
                new_w = w + 1;
            }

            ui.add_space(8.0);

            ui.label("H:");
            if ui.add_enabled(h > 1, egui::Button::new("−")).clicked() {
                new_h = h - 1;
            }
            ui.label(format!("{h:>2}"));
            if ui.add_enabled(h < 255, egui::Button::new("+")).clicked() {
                new_h = h + 1;
            }
        });
        if new_w != w || new_h != h {
            self.resize_level(new_w, new_h);
        }

        ui.separator();

        // --- Goal ---
        ui.heading("Goal");
        let is_collect = matches!(self.levels[self.current].goal, RawGoal::Collect { .. });
        ui.horizontal(|ui| {
            if ui.radio(is_collect, "Collect").clicked() && !is_collect {
                self.history.snapshot(&self.levels);
                clear_goal_chests(&mut self.levels[self.current]);
                self.levels[self.current].goal = RawGoal::Collect { x: 1, z: 1 };
                self.goal_pick_mode = true;
                if self.pending_chest_kind.is_goal() {
                    self.pending_chest = None;
                }
                self.dirty = true;
            }
            if ui.radio(!is_collect, "Mine All Gold").clicked() && is_collect {
                self.history.snapshot(&self.levels);
                clear_goal_chests(&mut self.levels[self.current]);
                let gold_blocks = count_gold(&self.levels[self.current]);
                self.levels[self.current].goal = RawGoal::MineAllGold(gold_blocks);
                self.goal_pick_mode = false;
                if self.pending_chest_kind.is_goal() {
                    self.pending_chest = None;
                }
                self.dirty = true;
            }
        });

        if let RawGoal::Collect { x, z } = self.levels[self.current].goal {
            if self.goal_pick_mode {
                ui.colored_label(
                    egui::Color32::from_rgb(140, 70, 0),
                    "Click a grid cell to place the GoalChest and choose its item…",
                );
                if ui.button("Cancel").clicked() {
                    self.goal_pick_mode = false;
                    let has_goal_chest = {
                        let level = &self.levels[self.current];
                        let (xi, zi) = (x as usize, z as usize);
                        zi < level.grid.len()
                            && xi < level.grid[zi].len()
                            && is_goal_id(level.grid[zi][xi])
                    };
                    if !has_goal_chest {
                        let gold_blocks = count_gold(&self.levels[self.current]);
                        self.levels[self.current].goal = RawGoal::MineAllGold(gold_blocks);
                    }
                    self.dirty = true;
                }
            } else {
                ui.horizontal(|ui| {
                    ui.label(format!("at ({x}, {z})"));
                    if ui.button("Pick on grid").clicked() {
                        self.goal_pick_mode = true;
                    }
                });
            }
        }
    }

    fn show_canvas(&mut self, ui: &mut egui::Ui) {
        if self.levels.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Create a level to begin.");
            });
            return;
        }

        let avail = ui.available_size();
        let (response, painter) = ui.allocate_painter(avail, egui::Sense::click_and_drag());
        let viewport = response.rect;
        let painter = painter.with_clip_rect(viewport);

        let cell_size = 24.0_f32;
        let (w, h) = {
            let l = &self.levels[self.current];
            (l.width as f32, l.height as f32)
        };
        let full_w = w + 2.0;
        let full_h = h + 2.0;

        if self.pending_recenter {
            let grid_size = egui::vec2(full_w * cell_size, full_h * cell_size);
            let inner = viewport.size();
            if grid_size.x <= inner.x && grid_size.y <= inner.y {
                // Center when it fits
                self.pan = (inner - grid_size) * 0.5;
            } else {
                // Otherwise align to top-left with a small margin
                self.pan = egui::vec2(10.0, 10.0);
            }
            self.pending_recenter = false;
        }

        // --- Space+drag panning ---
        let space_held = ui.ctx().input(|i| i.key_down(egui::Key::Space));

        if space_held {
            if response.drag_started_by(egui::PointerButton::Primary) {
                let pos = response.interact_pointer_pos().unwrap_or(viewport.min);
                self.pan_drag_origin = Some((pos, self.pan));
            }
            if response.dragged_by(egui::PointerButton::Primary) {
                if let (Some((start_pos, start_pan)), Some(cur)) =
                    (self.pan_drag_origin, response.interact_pointer_pos())
                {
                    self.pan = start_pan + (cur - start_pos);
                }
            }
            if response.drag_stopped_by(egui::PointerButton::Primary) {
                self.pan_drag_origin = None;
            }
        }

        let content_origin = viewport.min + self.pan;
        let interior_origin = content_origin + egui::vec2(cell_size, cell_size);

        // --- Hover cell ---
        self.hover_cell = response.hover_pos().and_then(|pos| {
            let local = pos - interior_origin;
            let cx = (local.x / cell_size).floor() as i32;
            let cy = (local.y / cell_size).floor() as i32;
            if cx >= 0 && cy >= 0 && (cx as u32) < w as u32 && (cy as u32) < h as u32 {
                Some((cx as u8, cy as u8))
            } else {
                None
            }
        });

        // --- Interactions ---
        if self.goal_pick_mode && response.clicked_by(egui::PointerButton::Primary) {
            if let Some((x, z)) = self.hover_cell {
                if (x, z) != self.levels[self.current].spawn {
                    self.history.snapshot(&self.levels);
                    self.set_goal_chest(x, z);
                }
            }
        } else if !space_held {
            match self.tool {
                Tool::Paint | Tool::Erase => {
                    let clicked = response.clicked_by(egui::PointerButton::Primary);
                    let dragging = response.dragged_by(egui::PointerButton::Primary);
                    if response.drag_started_by(egui::PointerButton::Primary) || clicked {
                        self.history.snapshot(&self.levels);
                    }
                    if (clicked || dragging) && self.hover_cell.is_some() {
                        let (x, z) = self.hover_cell.unwrap();
                        self.apply_cell(x, z);
                    }
                }
                Tool::Rect | Tool::Line => {
                    if response.drag_started_by(egui::PointerButton::Primary) {
                        self.history.snapshot(&self.levels);
                        self.drag_start_cell = self.hover_cell;
                        self.drag_current_cell = self.hover_cell;
                        self.drag_is_active = true;
                    }
                    if response.dragged_by(egui::PointerButton::Primary) && self.drag_is_active {
                        self.drag_current_cell = self.hover_cell;
                    }
                    if response.drag_stopped_by(egui::PointerButton::Primary) && self.drag_is_active
                    {
                        if let (Some(a), Some(b)) = (self.drag_start_cell, self.drag_current_cell) {
                            match self.tool {
                                Tool::Rect => self.apply_rect(a, b),
                                Tool::Line => self.apply_line(a, b),
                                _ => {}
                            }
                        }
                        self.drag_start_cell = None;
                        self.drag_current_cell = None;
                        self.drag_is_active = false;
                    }
                }
                Tool::Fill => {
                    if response.clicked_by(egui::PointerButton::Primary) {
                        if let Some((x, z)) = self.hover_cell {
                            self.history.snapshot(&self.levels);
                            self.apply_fill(x, z);
                        }
                    }
                }
            }
        }

        // --- Draw grid ---
        let level = &self.levels[self.current];
        let wall_color = color_for_id(BlockType::Wall1.id());
        for bz in 0..(level.height as i32 + 2) {
            for bx in 0..(level.width as i32 + 2) {
                let cell_min =
                    content_origin + egui::vec2(bx as f32 * cell_size, bz as f32 * cell_size);
                let cell_rect =
                    egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
                if !viewport.intersects(cell_rect) {
                    continue;
                }
                let x = bx - 1;
                let z = bz - 1;
                let inside = x >= 0 && z >= 0 && x < level.width as i32 && z < level.height as i32;
                let color = if inside {
                    self.cell_color(level, x as u8, z as u8)
                } else {
                    wall_color
                };
                painter.rect_filled(cell_rect, 0.0, color);
                painter.rect_stroke(
                    cell_rect,
                    0.0,
                    egui::Stroke::new(0.5, egui::Color32::from_gray(60)),
                    egui::StrokeKind::Inside,
                );
            }
        }

        // --- Goal marker ---
        if let RawGoal::Collect { x, z } = level.goal {
            let cell_min = interior_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
            let cell_rect = egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
            if viewport.intersects(cell_rect) {
                let outline = egui::Stroke::new(8.0, egui::Color32::BLACK);
                let fill = egui::Stroke::new(2.5, egui::Color32::WHITE);
                let target = cell_rect.shrink(outline.width * 0.5);
                let diagonals = [
                    [target.left_top(), target.right_bottom()],
                    [target.right_top(), target.left_bottom()],
                ];
                let extend = (outline.width - fill.width) * 0.5;

                for diagonal in diagonals {
                    let [start, end] = diagonal;
                    let direction = (end - start).normalized();
                    painter.line_segment(
                        [start - direction * extend, end + direction * extend],
                        outline,
                    );
                }
                for diagonal in diagonals {
                    painter.line_segment(diagonal, fill);
                }
            }
        }

        // --- Rect/Line preview: render the result ---
        if matches!(self.tool, Tool::Rect | Tool::Line) && self.drag_is_active {
            if let (Some(a), Some(b)) = (self.drag_start_cell, self.drag_current_cell) {
                let cells = if self.tool == Tool::Rect {
                    let (x0, x1) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
                    let (z0, z1) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
                    let mut v = vec![];
                    for z in z0..=z1 {
                        for x in x0..=x1 {
                            v.push((x, z));
                        }
                    }
                    v
                } else {
                    let (mut x0, mut z0) = (a.0 as i32, a.1 as i32);
                    let (x1, z1) = (b.0 as i32, b.1 as i32);
                    let dx = (x1 - x0).abs();
                    let sx = if x0 < x1 { 1 } else { -1 };
                    let dz = -(z1 - z0).abs();
                    let sz = if z0 < z1 { 1 } else { -1 };
                    let mut err = dx + dz;
                    let mut v = vec![];
                    loop {
                        v.push((x0 as u8, z0 as u8));
                        if x0 == x1 && z0 == z1 {
                            break;
                        }
                        let e2 = 2 * err;
                        if e2 >= dz {
                            err += dz;
                            x0 += sx;
                        }
                        if e2 <= dx {
                            err += dx;
                            z0 += sz;
                        }
                    }
                    v
                };
                let preview_color = color(self.selected_brush);
                for (x, z) in cells {
                    let cell_min =
                        interior_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
                    let cell_rect =
                        egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
                    if !viewport.intersects(cell_rect) {
                        continue;
                    }
                    painter.rect_filled(cell_rect, 0.0, preview_color);
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::from_gray(180)),
                        egui::StrokeKind::Inside,
                    );
                }
            }
        }

        // --- Hover highlight ---
        if let Some((x, z)) = self.hover_cell {
            let cell_min = interior_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
            let cell_rect = egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
            painter.rect_stroke(
                cell_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::from_gray(20)),
                egui::StrokeKind::Inside,
            );
        }

        // --- Hover indicator ---
        let (coords_text, desc_text) = match self.hover_cell {
            Some((x, z)) => {
                let desc = if (x, z) == level.spawn {
                    "PlayerSpawn".to_string()
                } else if let Some((_, id)) = level.chests.iter().find(|(c, _)| *c == (x, z)) {
                    let name = get_item(*id)
                        .map(|item| item.name())
                        .unwrap_or("unknown item");
                    format!("Chest — {name}")
                } else {
                    let id = level.grid[z as usize][x as usize];
                    name_for_id(id)
                        .map(|name| name.to_string())
                        .unwrap_or(format!("id {id}"))
                };
                (format!("({x}, {z})"), Some(desc))
            }
            None => ("(outside grid)".to_string(), None),
        };

        let font = egui::FontId::proportional(14.0);
        let text_color = egui::Color32::BLACK;
        let coords_galley = painter.layout_no_wrap(coords_text, font.clone(), text_color);
        let desc_galley = desc_text.map(|desc| painter.layout_no_wrap(desc, font, text_color));
        let line_height = coords_galley.size().y;
        let text_min = viewport.min + egui::vec2(2.0, 2.0);
        painter.galley(text_min, coords_galley, text_color);
        if let Some(desc_galley) = desc_galley {
            painter.galley(
                text_min + egui::vec2(0.0, line_height),
                desc_galley,
                text_color,
            );
        }
    }

    fn commit_chest(&mut self) {
        let Some((cx, cz)) = self.pending_chest else {
            return;
        };
        let kind = self.pending_chest_kind;
        let item = self.pending_chest_item;
        let level = &mut self.levels[self.current];
        if cx >= level.width || cz >= level.height || (cx, cz) == level.spawn {
            self.pending_chest = None;
            return;
        }
        if kind.is_goal() {
            if goal_cell(level) != Some((cx, cz)) {
                self.pending_chest = None;
                return;
            }
            clear_goal_chests(level);
        }
        put_cell(level, cx, cz, kind.id());
        level.chests.retain(|(c, _)| *c != (cx, cz));
        level.chests.push(((cx, cz), item));
        self.dirty = true;
        self.pending_chest = None;
    }

    fn show_chest_window(&mut self, ctx: &egui::Context) {
        let Some((cx, cz)) = self.pending_chest else {
            return;
        };
        let mut open = true;
        egui::Window::new("Chest item")
            .id(egui::Id::new("chest_item_window"))
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label(format!(
                    "Item for {} at ({cx}, {cz}):",
                    self.pending_chest_kind.name()
                ));
                egui::ScrollArea::vertical()
                    .max_height(220.0)
                    .show(ui, |ui| {
                        for entry in ITEMS {
                            let selected = self.pending_chest_item == entry.id();
                            let response = ui.selectable_label(selected, entry.name());
                            if response.clicked() {
                                self.pending_chest_item = entry.id();
                            }
                            if response.double_clicked() {
                                self.pending_chest_item = entry.id();
                                self.commit_chest();
                            }
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.commit_chest();
                    }
                    if ui.button("Cancel").clicked() {
                        self.pending_chest = None;
                    }
                });
            });
        if !open {
            self.pending_chest = None;
        }
    }
}

impl eframe::App for LevelEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        if let Some(at) = self.pending_delete_at {
            let elapsed = at.elapsed();
            if elapsed >= DELETE_CONFIRM_TIMEOUT {
                self.pending_delete_at = None;
            } else {
                ctx.request_repaint_after(DELETE_CONFIRM_TIMEOUT - elapsed);
            }
        }

        let mut delete_clicked = false;

        // Reduce CPU: no feathering on shape edges.
        ctx.tessellation_options_mut(|opt| {
            opt.feathering = false;
        });

        if ctx.input(|i| i.viewport().close_requested()) && self.dirty {
            self.save();
        }

        if !ctx.egui_wants_keyboard_input() {
            let undo = ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Z));
            let redo1 = ctx.input_mut(|i| {
                i.consume_key(
                    egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                    egui::Key::Z,
                )
            });
            let redo2 = ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::Y));
            let save = ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
            if undo {
                self.history.undo(&mut self.levels);
                self.dirty = true;
            }
            if redo1 || redo2 {
                self.history.redo(&mut self.levels);
                self.dirty = true;
            }
            if save {
                self.save();
            }
        }

        // ---- Top row: level / file / history ----
        egui::Panel::top("top").resizable(false).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Level:");
                if ui
                    .add_enabled(self.current > 0, egui::Button::new("◀"))
                    .clicked()
                {
                    self.previous_level();
                }
                if !self.levels.is_empty() {
                    let previous = self.current;
                    let text = format!("Level {}", previous + 1);
                    egui::ComboBox::from_id_salt("level_picker")
                        .selected_text(text)
                        .show_ui(ui, |ui| {
                            for i in 0..self.levels.len() {
                                let t = format!("Level {}", i + 1);
                                ui.selectable_value(&mut self.current, i, t);
                            }
                        });
                    if self.current != previous {
                        self.pending_recenter = true;
                        self.pending_delete_at = None;
                        self.pending_chest = None;
                        self.goal_pick_mode = false;
                    }
                }
                if ui
                    .add_enabled(self.current + 1 < self.levels.len(), egui::Button::new("▶"))
                    .clicked()
                {
                    self.next_level();
                }
                if ui.button("+ New").clicked() {
                    self.new_level();
                }
                if ui.button("+ Insert Before").clicked() {
                    self.insert_level_before_current();
                }
                if ui.button("+ Insert After").clicked() {
                    self.insert_level_after_current();
                }

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);

                delete_clicked = ui
                    .add_enabled(self.levels.len() > 1, egui::Button::new("Delete"))
                    .clicked();
                if delete_clicked {
                    if self.pending_delete_at.is_some() {
                        self.delete_level();
                    } else {
                        self.pending_delete_at = Some(Instant::now());
                    }
                }
                if self.pending_delete_at.is_some() {
                    ui.colored_label(egui::Color32::RED, "Are you sure?");
                }

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);
                if ui.button("Save").clicked() {
                    self.save();
                }

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);
                if ui.button("Undo").clicked() {
                    self.history.undo(&mut self.levels);
                    self.dirty = true;
                }
                if ui.button("Redo").clicked() {
                    self.history.redo(&mut self.levels);
                    self.dirty = true;
                }

                // ----- Right group -----
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Rightmost first
                    if let Some(t) = self.saved_at {
                        let elapsed = t.elapsed();
                        if elapsed < SAVED_VISIBLE {
                            ui.colored_label(egui::Color32::from_rgb(120, 200, 120), "Saved");
                            ctx.request_repaint_after(SAVED_VISIBLE - elapsed);
                        } else {
                            self.saved_at = None;
                        }
                    }
                    if self.dirty {
                        ui.colored_label(egui::Color32::YELLOW, "●");
                    }
                });
            });
        });

        // ---- Second top row: tools ----
        egui::Panel::top("tools").resizable(false).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                ui.selectable_value(&mut self.tool, Tool::Paint, "Paint");
                ui.selectable_value(&mut self.tool, Tool::Erase, "Erase");
                ui.selectable_value(&mut self.tool, Tool::Rect, "Rect");
                ui.selectable_value(&mut self.tool, Tool::Line, "Line");
                ui.selectable_value(&mut self.tool, Tool::Fill, "Fill");
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);
                if ui.button("Home").clicked() {
                    self.pending_recenter = true;
                }
                ui.label("Hold Space + drag to pan");
            });
        });

        egui::Panel::left("palette")
            .resizable(false)
            .default_size(180.0)
            .show(ui, |ui| {
                self.show_palette(ui);
            });

        egui::Panel::right("meta")
            .resizable(false)
            .default_size(240.0)
            .show(ui, |ui| {
                self.show_metadata(ui);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            self.show_canvas(ui);
        });

        self.show_chest_window(&ctx);

        if self.pending_delete_at.is_some()
            && !delete_clicked
            && ctx.input(|i| i.pointer.any_click())
        {
            self.pending_delete_at = None;
        }
    }
}
