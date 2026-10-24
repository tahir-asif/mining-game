use crate::ItemId;
use crate::block::{BlockType, blocktype_from_id};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum StarCount {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl StarCount {
    pub const MAX: Self = Self::Three;

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Two,
            Self::Two | Self::Three => Self::Three,
        }
    }
}

impl TryFrom<u8> for StarCount {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(format!("invalid star count: {value}")),
        }
    }
}

impl From<StarCount> for u8 {
    fn from(value: StarCount) -> Self {
        value.as_u8()
    }
}

pub const STAR_CHESTS_PER_LEVEL: u8 = StarCount::Three.as_u8();

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawLevel {
    pub spawn: (u8, u8),
    pub width: u8,
    pub height: u8,
    pub goal: RawGoal,
    pub grid: Vec<Vec<u8>>,
    pub chests: Vec<((u8, u8), ItemId)>,
}

impl RawLevel {
    pub fn star_chest_count(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&id| blocktype_from_id(id) == Some(BlockType::StarChest))
            .count()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum RawGoal {
    Collect { x: u8, z: u8 },
    MineAllGold(u16),
}

pub type BlockId = u8;
