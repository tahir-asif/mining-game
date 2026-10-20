use macroquad::prelude::*;
use shared::{BlockCategory, BlockType};

#[derive(PartialEq)]
pub enum DropKind {
    Gold,
    Energy,
    Item,
}

pub struct Drop {
    pub kind: DropKind,
    pub amount: u16,
}

impl Drop {
    pub const fn new(kind: DropKind, amount: u16) -> Self {
        Drop { kind, amount }
    }
}

pub enum MiningOutcome {
    Unbreakable,
    Damaged,
    Destroyed,
    Gained(&'static [Drop]),
}

#[derive(Clone, Copy)]
pub struct Block {
    kind: BlockType,
    health: Option<u16>,
}

impl Block {
    pub fn new(kind: BlockType) -> Self {
        Block {
            kind,
            health: stats(kind).max_health,
        }
    }

    pub fn mine(&mut self, power: u16) -> MiningOutcome {
        match stats(self.kind).category {
            BlockCategory::Torch => MiningOutcome::Unbreakable,
            BlockCategory::Wall => MiningOutcome::Unbreakable,
            BlockCategory::Crystal => MiningOutcome::Gained(stats(self.kind).drops),
            BlockCategory::Chest => MiningOutcome::Gained(stats(self.kind).drops),
            BlockCategory::Rock => {
                if let Some(hp) = self.health {
                    let after = hp.saturating_sub(power);
                    if after == 0 {
                        return MiningOutcome::Destroyed;
                    }
                    self.health = Some(after);
                }
                MiningOutcome::Damaged
            }
            BlockCategory::GoldOre => {
                if let Some(hp) = self.health {
                    let after = hp.saturating_sub(power);
                    if after == 0 {
                        return MiningOutcome::Gained(stats(self.kind).drops);
                    }
                    self.health = Some(after);
                }
                MiningOutcome::Damaged
            }
        }
    }

    pub fn colour(&self) -> Color {
        stats(self.kind).colour
    }
}

fn stats(block: BlockType) -> &'static BlockDef {
    match block {
        BlockType::Torch => &TORCH,
        BlockType::GoalChest => &GOALCHEST,
        BlockType::StarChest => &STARCHEST,
        BlockType::ItemChest => &ITEMCHEST,
        BlockType::Wall1 => &WALL1,
        BlockType::EnergyCrystal1 => &ENERGYCRYSTAL1,
        BlockType::TechCrystal1 => &TECHCRYSTAL1,
        BlockType::GoldOre1 => &GOLDORE1,
        BlockType::Rock1 => &ROCK1,
        BlockType::Rock2 => &ROCK2,
        BlockType::Rock3 => &ROCK3,
        BlockType::Rock4 => &ROCK4,
        BlockType::Rock5 => &ROCK5,
        BlockType::Rock6 => &ROCK6,
        BlockType::Rock7 => &ROCK7,
        BlockType::Rock8 => &ROCK8,
        BlockType::Rock9 => &ROCK9,
        BlockType::Rock10 => &ROCK10,
    }
}

struct BlockDef {
    category: BlockCategory,
    max_health: Option<u16>,
    colour: Color,
    drops: &'static [Drop],
}

static TORCH: BlockDef = BlockDef {
    category: BlockCategory::Torch,
    max_health: None,
    colour: Color::from_rgba(255, 150, 40, 255),
    drops: &[],
};

static GOALCHEST: BlockDef = BlockDef {
    category: BlockCategory::Chest,
    max_health: None,
    colour: Color::from_rgba(90, 200, 120, 255),
    drops: &[Drop::new(DropKind::Item, 1)],
};

static STARCHEST: BlockDef = BlockDef {
    category: BlockCategory::Chest,
    max_health: None,
    colour: Color::from_rgba(80, 150, 255, 255),
    drops: &[],
};

static ITEMCHEST: BlockDef = BlockDef {
    category: BlockCategory::Chest,
    max_health: None,
    colour: Color::from_rgba(255, 220, 40, 255),
    drops: &[Drop::new(DropKind::Item, 1)],
};

static WALL1: BlockDef = BlockDef {
    category: BlockCategory::Wall,
    max_health: None,
    colour: Color::from_rgba(50, 50, 50, 255),
    drops: &[],
};

static GOLDORE1: BlockDef = BlockDef {
    category: BlockCategory::GoldOre,
    max_health: Some(4),
    colour: Color::from_rgba(230, 190, 50, 255),
    drops: &[Drop::new(DropKind::Gold, 5)],
};

static ENERGYCRYSTAL1: BlockDef = BlockDef {
    category: BlockCategory::Crystal,
    max_health: None,
    colour: Color::from_rgba(220, 60, 60, 255),
    drops: &[Drop::new(DropKind::Energy, 10)],
};

static TECHCRYSTAL1: BlockDef = BlockDef {
    category: BlockCategory::Crystal,
    max_health: None,
    colour: Color::from_rgba(60, 100, 220, 255),
    drops: &[Drop::new(DropKind::Energy, 10)],
};

static ROCK1: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(1),
    colour: Color::from_rgba(238, 228, 212, 255),
    drops: &[],
};

static ROCK2: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(3),
    colour: Color::from_rgba(226, 211, 190, 255),
    drops: &[],
};

static ROCK3: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(7),
    colour: Color::from_rgba(212, 192, 166, 255),
    drops: &[],
};

static ROCK4: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(10),
    colour: Color::from_rgba(196, 171, 141, 255),
    drops: &[],
};

static ROCK5: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(25),
    colour: Color::from_rgba(178, 149, 116, 255),
    drops: &[],
};

static ROCK6: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(55),
    colour: Color::from_rgba(157, 126, 94, 255),
    drops: &[],
};

static ROCK7: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(80),
    colour: Color::from_rgba(135, 104, 75, 255),
    drops: &[],
};

static ROCK8: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(110),
    colour: Color::from_rgba(112, 83, 58, 255),
    drops: &[],
};

static ROCK9: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(145),
    colour: Color::from_rgba(88, 63, 42, 255),
    drops: &[],
};

static ROCK10: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(185),
    colour: Color::from_rgba(66, 46, 30, 255),
    drops: &[],
};
