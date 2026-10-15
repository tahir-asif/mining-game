use macroquad::prelude::*;
use shared::BlockType;

pub enum DropKind {
    Gold,
    Energy,
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
enum BlockCategory {
    Wall,
    Rock,
    Ore,
    Crystal,
    Chest,
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
            BlockCategory::Ore => {
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
        BlockType::Wall1 => &WALL_1,
        BlockType::Rock1 => &ROCK_1,
        BlockType::Rock2 => &ROCK_2,
        BlockType::Rock3 => &ROCK_3,
        BlockType::GoldOre1 => &GOLD_ORE_1,
        BlockType::EnergyCrystal1 => &ENERGY_CRYSTAL_1,
        BlockType::TechCrystal1 => &TECH_CRYSTAL_1,
        BlockType::Chest1 => &CHEST_1,
    }
}

struct BlockDef {
    category: BlockCategory,
    max_health: Option<u16>,
    colour: Color,
    drops: &'static [Drop],
}

static WALL_1: BlockDef = BlockDef {
    category: BlockCategory::Wall,
    max_health: None,
    colour: Color::from_rgba(25, 25, 30, 255),
    drops: &[],
};

static ROCK_1: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(1),
    colour: Color::from_rgba(190, 150, 110, 255),
    drops: &[],
};

static ROCK_2: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(5),
    colour: Color::from_rgba(150, 110, 80, 255),
    drops: &[],
};

static ROCK_3: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(10),
    colour: Color::from_rgba(110, 80, 60, 255),
    drops: &[],
};

static GOLD_ORE_1: BlockDef = BlockDef {
    category: BlockCategory::Ore,
    max_health: Some(4),
    colour: Color::from_rgba(230, 190, 50, 255),
    drops: &[Drop::new(DropKind::Gold, 5)],
};

static ENERGY_CRYSTAL_1: BlockDef = BlockDef {
    category: BlockCategory::Crystal,
    max_health: None,
    colour: Color::from_rgba(220, 60, 60, 255),
    drops: &[Drop::new(DropKind::Energy, 10)],
};

static TECH_CRYSTAL_1: BlockDef = BlockDef {
    category: BlockCategory::Crystal,
    max_health: None,
    colour: Color::from_rgba(60, 100, 220, 255),
    drops: &[Drop::new(DropKind::Energy, 10)],
};

static CHEST_1: BlockDef = BlockDef {
    category: BlockCategory::Chest,
    max_health: None,
    colour: Color::from_rgba(255, 220, 40, 255),
    drops: &[],
};
