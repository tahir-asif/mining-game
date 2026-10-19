use shared::ItemId;

use crate::level::MapCoords;

pub struct LevelData {
    pub id: LevelId,
    pub spawn: MapCoords,
    pub width: u8,
    pub height: u8,
    pub goal: Goal,
    pub block_ids: &'static [BlockId],
    pub chests: &'static [(MapCoords, ItemId)],
}

pub fn get_level(id: LevelId) -> Option<&'static LevelData> {
    let id = usize::from(id);
    LEVELS.get(id).copied()
}

#[derive(Clone, Copy)]
pub enum Goal {
    Collect(MapCoords),
    MineAllGold,
}

impl Goal {
    pub const fn to_str(self) -> &'static str {
        match self {
            Goal::Collect(_) => "!  Collect an item",
            Goal::MineAllGold => "!  Mine all gold",
        }
    }
}

pub type LevelId = u8;
pub type BlockId = u8;

include!(concat!(env!("OUT_DIR"), "/levels_generated.rs"));
