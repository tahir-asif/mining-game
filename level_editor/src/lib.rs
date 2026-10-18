#![recursion_limit = "256"]

mod palette;

use eframe::egui;
use palette::{Brush, all_brushes, color, color_for_id, label, name_for_id};
use shared::block::{BlockType, ID_EMPTY};
use shared::items::get_item;
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
    if id != ID_EMPTY && (x, z) == level.spawn {
        return;
    }
    level.grid[z as usize][x as usize] = id;
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
    pending_chest_item: ItemId,

    // Status / save / delete
    saved_at: Option<Instant>,
    pending_delete: bool,

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
            pending_chest_item: ITEMS.first().map_or(0, |entry| entry.id),
            saved_at: None,
            pending_delete: false,
            history: History::new(),
        }
    }

    fn save(&mut self) {
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

    fn new_level(&mut self) {
        self.history.snapshot(&self.levels);
        let next_id = self.levels.iter().map(|l| l.id).max().map_or(0, |m| m + 1);
        let (w, h) = (20u8, 15u8);
        let wid = BlockType::Wall1.id();
        let mut grid = vec![vec![ID_EMPTY; w as usize]; h as usize];
        for x in 0..w as usize {
            grid[0][x] = wid;
            grid[(h as usize) - 1][x] = wid;
        }
        for z in 0..h as usize {
            grid[z][0] = wid;
            grid[z][(w as usize) - 1] = wid;
        }
        self.levels.push(RawLevel {
            id: next_id,
            spawn: (1, 1),
            width: w,
            height: h,
            goal: RawGoal::MineAllGold,
            grid,
            chests: vec![],
        });
        self.current = self.levels.len() - 1;
        self.dirty = true;
        self.pan = egui::Vec2::ZERO;
    }

    fn resize_level(&mut self, new_w: u8, new_h: u8) {
        if self.levels.is_empty() {
            return;
        }
        let level = &mut self.levels[self.current];
        let old_w = level.width as usize;
        let old_h = level.height as usize;
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
        if let RawGoal::Collect { x, z } = &mut level.goal {
            *x = (*x).min(new_w.saturating_sub(1));
            *z = (*z).min(new_h.saturating_sub(1));
        }
        level.chests.retain(|((x, z), _)| *x < new_w && *z < new_h);
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
        self.pending_delete = false;
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
            Brush::Block(BlockType::Chest1) => {
                if let Some((_, id)) = level.chests.iter().find(|(c, _)| *c == (x, z)) {
                    self.pending_chest_item = *id;
                }
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
            Brush::Spawn | Brush::Block(BlockType::Chest1) => return,
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
            put_cell(level, cx as u8, cz as u8, new_id);
            if new_id == ID_EMPTY {
                level.chests.retain(|(c, _)| *c != (cx as u8, cz as u8));
            }
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

        // --- Hover ---
        ui.heading("Hover");
        if let Some((x, z)) = self.hover_cell {
            let level = &self.levels[self.current];
            let desc = if (x, z) == level.spawn {
                "PlayerSpawn".to_string()
            } else if let Some((_, id)) = level.chests.iter().find(|(c, _)| *c == (x, z)) {
                let name = get_item(*id).map(|item| item.name()).unwrap_or("unknown item");
                format!("Chest — {name}")
            } else {
                let id = level.grid[z as usize][x as usize];
                name_for_id(id)
                    .map(|name| name.to_string())
                    .unwrap_or(format!("id {id}"))
            };
            ui.label(format!("({x}, {z}) — {desc}"));
        } else {
            ui.label("(outside grid)");
        }

        ui.separator();

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

        // ID
        let mut id = self.levels[self.current].id;
        ui.horizontal(|ui| {
            ui.label("ID:");
            if ui
                .add(egui::DragValue::new(&mut id).range(0..=255u32))
                .changed()
            {
                self.levels[self.current].id = id;
                self.dirty = true;
            }
        });

        ui.separator();

        // --- Goal ---
        ui.heading("Goal");
        let is_collect = matches!(self.levels[self.current].goal, RawGoal::Collect { .. });
        ui.horizontal(|ui| {
            if ui.radio(is_collect, "Collect").clicked() && !is_collect {
                self.levels[self.current].goal = RawGoal::Collect { x: 1, z: 1 };
                self.goal_pick_mode = true;
                self.dirty = true;
            }
            if ui.radio(!is_collect, "Mine All Gold").clicked() && is_collect {
                self.levels[self.current].goal = RawGoal::MineAllGold;
                self.goal_pick_mode = false;
                self.dirty = true;
            }
        });

        if let RawGoal::Collect { x, z } = self.levels[self.current].goal {
            if self.goal_pick_mode {
                ui.colored_label(
                    egui::Color32::from_rgb(140, 70, 0),
                    "Click a grid cell to set goal…",
                );
                if ui.button("Cancel").clicked() {
                    self.goal_pick_mode = false;
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

        if self.pending_recenter {
            let grid_size = egui::vec2(w * cell_size, h * cell_size);
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

        // --- Hover cell ---
        self.hover_cell = response.hover_pos().and_then(|pos| {
            let local = pos - content_origin;
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
                self.history.snapshot(&self.levels);
                self.levels[self.current].goal = RawGoal::Collect { x, z };
                self.goal_pick_mode = false;
                self.dirty = true;
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
        for z in 0..level.height {
            for x in 0..level.width {
                let cell_min =
                    content_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
                let cell_rect =
                    egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
                if !viewport.intersects(cell_rect) {
                    continue;
                }
                let color = self.cell_color(level, x, z);
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
            let cell_min = content_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
            let cell_rect = egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
            if viewport.intersects(cell_rect) {
                painter.text(
                    cell_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "G",
                    egui::FontId::proportional(cell_size * 0.65),
                    egui::Color32::from_rgb(240, 90, 200),
                );
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
                        content_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
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
            let cell_min = content_origin + egui::vec2(x as f32 * cell_size, z as f32 * cell_size);
            let cell_rect = egui::Rect::from_min_size(cell_min, egui::vec2(cell_size, cell_size));
            painter.rect_stroke(
                cell_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::from_gray(20)),
                egui::StrokeKind::Inside,
            );
        }
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
                ui.label(format!("Item for chest at ({cx}, {cz}):"));
                egui::ScrollArea::vertical()
                    .max_height(220.0)
                    .show(ui, |ui| {
                        for entry in ITEMS {
                            let selected = self.pending_chest_item == entry.id;
                            if ui.selectable_label(selected, entry.item.name()).clicked() {
                                self.pending_chest_item = entry.id;
                            }
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        let level = &mut self.levels[self.current];
                        if (cx, cz) != level.spawn {
                            put_cell(level, cx, cz, BlockType::Chest1.id());
                            level.chests.retain(|(c, _)| *c != (cx, cz));
                            level.chests.push(((cx, cz), self.pending_chest_item));
                            self.dirty = true;
                        }
                        self.pending_chest = None;
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
                if !self.levels.is_empty() {
                    let selected = self.current;
                    let text = format!("Level {} (id {})", selected, self.levels[selected].id);
                    egui::ComboBox::from_id_salt("level_picker")
                        .selected_text(text)
                        .show_ui(ui, |ui| {
                            for i in 0..self.levels.len() {
                                let t = format!("Level {} (id {})", i, self.levels[i].id);
                                ui.selectable_value(&mut self.current, i, t);
                            }
                        });
                }
                if ui.button("+ New").clicked() {
                    self.new_level();
                }
                if ui.button("Delete").clicked() {
                    self.pending_delete = true;
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

        if self.pending_delete {
            let mut open = true;
            egui::Window::new("Delete level?")
                .id(egui::Id::new("delete_confirm"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(&ctx, |ui| {
                    ui.label("This can be undone with Ctrl+Z.");
                    ui.horizontal(|ui| {
                        if ui.button("Delete").clicked() {
                            self.delete_level();
                        }
                        if ui.button("Cancel").clicked() {
                            self.pending_delete = false;
                        }
                    });
                });
            if !open {
                self.pending_delete = false;
            }
        }
    }
}
