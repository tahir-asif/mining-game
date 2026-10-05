use crate::level::MapCoords;

pub struct LevelData {
    pub id: LevelId,
    pub spawn: MapCoords,
    pub goal: Goal,
    pub prerequisies: &'static [LevelId],
}

pub enum Goal {
    Collect(MapCoords),
    MineAllGold,
}

pub type LevelId = u8;
