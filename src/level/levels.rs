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
    LEVELS.get(id - 1).copied()
}

#[derive(Clone, Copy)]
pub enum Goal {
    Collect(MapCoords),
    MineAllGold(u16),
}

impl Goal {
    pub fn to_str(self, gold_mined: u16) -> String {
        match self {
            Goal::Collect(_) => format!("!  Find the exit chest"),
            Goal::MineAllGold(total) => format!("!  Mine all gold {gold_mined}/{total}"),
        }
    }
}

pub type LevelId = u8;
pub type BlockId = u8;

include!(concat!(env!("OUT_DIR"), "/levels_generated.rs"));
