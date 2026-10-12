use egui::Color32;
use shared::block::{BlockType, blocktype_from_id};
use strum::IntoEnumIterator;

pub use shared::block::ID_EMPTY;

#[derive(Clone, Copy)]
pub enum Brush {
    Empty,
    Spawn,
    Block(BlockType),
}

impl PartialEq for Brush {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Brush::Empty, Brush::Empty) | (Brush::Spawn, Brush::Spawn) => true,
            (Brush::Block(a), Brush::Block(b)) => a.id() == b.id(),
            _ => false,
        }
    }
}

impl Brush {
    pub fn block_id(self) -> Option<u8> {
        match self {
            Brush::Empty => Some(ID_EMPTY),
            Brush::Spawn => None,
            Brush::Block(kind) => Some(kind.id()),
        }
    }
}

pub fn all_brushes() -> Vec<Brush> {
    let mut brushes = vec![Brush::Empty, Brush::Spawn];
    brushes.extend(BlockType::iter().map(Brush::Block));
    brushes
}

pub fn label(brush: Brush) -> &'static str {
    match brush {
        Brush::Empty => "Empty",
        Brush::Spawn => "Player Spawn",
        Brush::Block(kind) => kind.name(),
    }
}

pub const fn color(brush: Brush) -> Color32 {
    match brush {
        Brush::Empty => Color32::from_rgb(235, 235, 235),
        Brush::Spawn => Color32::from_rgb(80, 220, 120),
        Brush::Block(kind) => block_color(kind),
    }
}

pub const fn block_color(kind: BlockType) -> Color32 {
    match kind {
        BlockType::Wall1 => Color32::from_rgb(25, 25, 30),
        BlockType::Chest1 => Color32::from_rgb(255, 220, 40),
        BlockType::EnergyCrystal1 => Color32::from_rgb(220, 60, 60),
        BlockType::TechCrystal1 => Color32::from_rgb(60, 100, 220),
        BlockType::GoldOre1 => Color32::from_rgb(230, 190, 50),
        BlockType::Rock1 => Color32::from_rgb(190, 150, 110),
        BlockType::Rock2 => Color32::from_rgb(150, 110, 80),
        BlockType::Rock3 => Color32::from_rgb(110, 80, 60),
    }
}

pub fn color_for_id(id: u8) -> Color32 {
    if id == ID_EMPTY {
        return color(Brush::Empty);
    }
    blocktype_from_id(id)
        .map(block_color)
        .unwrap_or(Color32::MAGENTA)
}

pub fn name_for_id(id: u8) -> Option<&'static str> {
    if id == ID_EMPTY {
        return Some("Empty");
    }
    blocktype_from_id(id).map(BlockType::name)
}
