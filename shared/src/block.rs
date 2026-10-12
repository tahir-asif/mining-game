use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

pub const ID_EMPTY: u8 = 0;

#[derive(Clone, Copy, EnumIter, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum BlockType {
    Wall1 = 1,
    Chest1 = 2,
    EnergyCrystal1 = 3,
    TechCrystal1 = 4,
    GoldOre1 = 5,
    Rock1 = 6,
    Rock2 = 7,
    Rock3 = 8,
}

impl BlockType {
    pub fn id(self) -> u8 {
        self.into()
    }

    pub const fn name(self) -> &'static str {
        match self {
            BlockType::Wall1 => "Wall 1",
            BlockType::Chest1 => "Chest 1",
            BlockType::EnergyCrystal1 => "Energy Crystal 1",
            BlockType::TechCrystal1 => "Tech Crystal 1",
            BlockType::GoldOre1 => "Gold Ore 1",
            BlockType::Rock1 => "Rock 1",
            BlockType::Rock2 => "Rock 2",
            BlockType::Rock3 => "Rock 3",
        }
    }
}

pub fn blocktype_from_id(id: u8) -> Option<BlockType> {
    match BlockType::try_from(id) {
        Ok(block) => Some(block),
        Err(_) => None,
    }
}
