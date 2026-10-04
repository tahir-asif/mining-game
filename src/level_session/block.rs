use macroquad::prelude::*;

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
pub enum BlockType {
    Wall1,
    Rock1,
    Rock2,
    Rock3,
    Ore1,
    Crystal1,
    Chest1,
}

impl BlockType {
    fn def(&self) -> &'static BlockDef {
        match self {
            BlockType::Wall1 => &WALL_1,
            BlockType::Rock1 => &ROCK_1,
            BlockType::Rock2 => &ROCK_2,
            BlockType::Rock3 => &ROCK_3,
            BlockType::Ore1 => &ORE_1,
            BlockType::Crystal1 => &CRYSTAL_1,
            BlockType::Chest1 => &CHEST_1,
        }
    }

    fn category(self) -> BlockCategory {
        self.def().category
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    kind: BlockType,
    health: Option<u16>,
}

impl Block {
    pub fn new(kind: BlockType) -> Self {
        Block {
            health: kind.def().max_health,
            kind,
        }
    }

    pub fn mine(&mut self, power: u16) -> MiningOutcome {
        match self.kind.category() {
            BlockCategory::Wall => MiningOutcome::Unbreakable,
            BlockCategory::Crystal => MiningOutcome::Gained(self.kind.def().drops),
            BlockCategory::Chest => MiningOutcome::Gained(self.kind.def().drops),
            BlockCategory::Rock => {
                if let Some(hp) = self.health {
                    let after = hp.saturating_sub(power);
                    self.health = Some(after);
                    if after == 0 {
                        return MiningOutcome::Destroyed;
                    }
                }
                MiningOutcome::Damaged
            }
            BlockCategory::Ore => {
                if let Some(hp) = self.health {
                    let after = hp.saturating_sub(power);
                    self.health = Some(after);
                    if after == 0 {
                        return MiningOutcome::Gained(self.kind.def().drops);
                    }
                }
                MiningOutcome::Damaged
            }
        }
    }

    pub fn colour(&self) -> Color {
        self.kind.def().colour
    }
}

// Every different types of blocks
struct BlockDef {
    category: BlockCategory,
    max_health: Option<u16>,
    colour: Color,
    drops: &'static [Drop],
}

static WALL_1: BlockDef = BlockDef {
    category: BlockCategory::Wall,
    max_health: None,
    colour: DARKGRAY,
    drops: &[],
};

static ROCK_1: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(1),
    colour: Color::new(0.0, 0.0, 0.7, 1.0),
    drops: &[],
};

static ROCK_2: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(5),
    colour: Color::new(0.7, 0.0, 0.0, 1.0),
    drops: &[],
};

static ROCK_3: BlockDef = BlockDef {
    category: BlockCategory::Rock,
    max_health: Some(10),
    colour: Color::new(0.0, 0.7, 0.0, 1.0),
    drops: &[],
};

static ORE_1: BlockDef = BlockDef {
    category: BlockCategory::Ore,
    max_health: Some(4),
    colour: GOLD,
    drops: &[Drop::new(DropKind::Gold, 5)],
};

static CRYSTAL_1: BlockDef = BlockDef {
    category: BlockCategory::Crystal,
    max_health: None,
    colour: RED,
    drops: &[Drop::new(DropKind::Energy, 10)],
};

static CHEST_1: BlockDef = BlockDef {
    category: BlockCategory::Chest,
    max_health: None,
    colour: BROWN,
    drops: &[],
};
