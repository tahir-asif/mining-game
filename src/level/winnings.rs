use shared::{ItemRef, StarCount};

pub struct Winnings {
    pub gold: u16,
    pub items: Vec<ItemRef>,
    pub latest_completed_level: u8,
    pub stars: StarCount,
}
