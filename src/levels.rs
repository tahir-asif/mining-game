use crate::level_session::coordinate::MapCoords;

pub struct LevelDef {
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
