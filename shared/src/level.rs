use crate::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawLevel {
    pub spawn: (u8, u8),
    pub width: u8,
    pub height: u8,
    pub goal: RawGoal,
    pub grid: Vec<Vec<u8>>,
    pub chests: Vec<((u8, u8), ItemId)>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum RawGoal {
    Collect { x: u8, z: u8 },
    MineAllGold,
}

pub type BlockId = u8;
