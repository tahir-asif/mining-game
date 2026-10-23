use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

pub const ID_EMPTY: u8 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockCategory {
    Wall,
    Rock,
    GoldOre,
    Crystal,
    Chest,
    Torch,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum BlockType {
    Torch = 1,
    GoalChest = 11,
    StarChest = 12,
    ItemChest = 13,
    Wall1 = 30,
    EnergyCrystal1 = 60,
    TechCrystal1 = 70,
    GoldOre1 = 80,
    Rock1 = 100,
    Rock2 = 101,
    Rock3 = 102,
    Rock4 = 103,
    Rock5 = 104,
    Rock6 = 105,
    Rock7 = 106,
    Rock8 = 107,
    Rock9 = 108,
    Rock10 = 109,
}

impl BlockType {
    pub fn id(self) -> u8 {
        self.into()
    }

    pub const fn name(self) -> &'static str {
        match self {
            BlockType::Torch => "Torch",
            BlockType::GoalChest => "GoalChest",
            BlockType::StarChest => "StarChest",
            BlockType::ItemChest => "ItemChest",
            BlockType::Wall1 => "Wall1",
            BlockType::EnergyCrystal1 => "EnergyCrystal1",
            BlockType::TechCrystal1 => "TechCrystal1",
            BlockType::GoldOre1 => "GoldOre1",
            BlockType::Rock1 => "Rock1",
            BlockType::Rock2 => "Rock2",
            BlockType::Rock3 => "Rock3",
            BlockType::Rock4 => "Rock4",
            BlockType::Rock5 => "Rock5",
            BlockType::Rock6 => "Rock6",
            BlockType::Rock7 => "Rock7",
            BlockType::Rock8 => "Rock8",
            BlockType::Rock9 => "Rock9",
            BlockType::Rock10 => "Rock10",
        }
    }

    pub const fn category(self) -> BlockCategory {
        match self {
            BlockType::Torch => BlockCategory::Torch,
            BlockType::GoalChest | BlockType::StarChest | BlockType::ItemChest => {
                BlockCategory::Chest
            }
            BlockType::Wall1 => BlockCategory::Wall,
            BlockType::EnergyCrystal1 | BlockType::TechCrystal1 => BlockCategory::Crystal,
            BlockType::GoldOre1 => BlockCategory::GoldOre,
            BlockType::Rock1
            | BlockType::Rock2
            | BlockType::Rock3
            | BlockType::Rock4
            | BlockType::Rock5
            | BlockType::Rock6
            | BlockType::Rock7
            | BlockType::Rock8
            | BlockType::Rock9
            | BlockType::Rock10 => BlockCategory::Rock,
        }
    }

    pub const fn is_goal(self) -> bool {
        matches!(self, BlockType::GoalChest)
    }

    pub const fn is_item_chest(self) -> bool {
        matches!(self, BlockType::ItemChest | BlockType::GoalChest)
    }
}

pub fn blocktype_from_id(id: u8) -> Option<BlockType> {
    match BlockType::try_from(id) {
        Ok(block) => Some(block),
        Err(_) => None,
    }
}
