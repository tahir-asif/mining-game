use crate::level::MapCoords;

pub struct LevelData {
    pub id: LevelId,
    pub spawn: MapCoords,
    pub width: u8,
    pub height: u8,
    pub goal: Goal,
    pub prerequisites: &'static [LevelId],
    pub block_ids: &'static [BlockId],
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

/*  Coordinates with respect to block_ids:
 *  0,0  1,0  2,0  3,0
 *  0,1  1,1
 *  0,2
 *  0,3
 */

pub static LEVELS: &[&LevelData] = &[&LEVEL_1, &LEVEL_2, &LEVEL_3];

pub static LEVEL_1: LevelData = LevelData {
    id: 0,
    spawn: MapCoords { x: 1, z: 1 },
    width: 12,
    height: 7,
    goal: Goal::Collect(MapCoords { x: 10, z: 3 }),
    prerequisites: &[],
    block_ids: &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 3, 4, 5, 6, 7, 0, 0, 5, 0, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 5, 5, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
};

pub static LEVEL_2: LevelData = LevelData {
    id: 1,
    spawn: MapCoords { x: 4, z: 3 },
    width: 8,
    height: 7,
    goal: Goal::Collect(MapCoords { x: 6, z: 3 }),
    prerequisites: &[1],
    block_ids: &[
        1, 1, 1, 1, 1, 1, 1, 1, 5, 6, 7, 0, 0, 5, 5, 1, 0, 0, 0, 0, 0, 5, 5, 1, 0, 0, 0, 0, 0, 0,
        7, 1, 0, 0, 0, 0, 0, 2, 2, 1, 0, 0, 0, 0, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
};

pub static LEVEL_3: LevelData = LevelData {
    id: 2,
    spawn: MapCoords { x: 2, z: 2 },
    width: 5,
    height: 5,
    goal: Goal::Collect(MapCoords { x: 3, z: 3 }),
    prerequisites: &[1],
    block_ids: &[
        1, 1, 1, 1, 1, 1, 1, 7, 6, 1, 1, 6, 0, 5, 1, 1, 1, 4, 6, 1, 1, 1, 1, 1, 1,
    ],
};
