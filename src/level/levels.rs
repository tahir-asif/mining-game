use crate::{
    items::{Item, Pickaxe},
    level::MapCoords,
    save::*,
};

pub struct LevelData {
    pub id: LevelId,
    pub spawn: MapCoords,
    pub width: u8,
    pub height: u8,
    pub goal: Goal,
    pub prerequisites: &'static [LevelId],
    pub block_ids: &'static [BlockId],
    pub chests: &'static [(MapCoords, &'static Pickaxe)],
}

pub fn level_data(id: LevelId) -> Option<&'static LevelData> {
    let id = usize::from(id);
    LEVELS.get(id).copied()
}

pub enum Goal {
    Collect(MapCoords),
    MineAllGold,
}

pub type LevelId = u8;
pub type BlockId = u8;

include!(concat!(env!("OUT_DIR"), "/levels_generated.rs"));
