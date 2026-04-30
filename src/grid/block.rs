use macroquad::prelude::*;

#[derive(Clone)]
pub enum MiningDrop {
    // TODO: implement MiningDrop drop properly
    Gold,
    Energy(u8),
    Item,
}

#[derive(Clone)]
pub enum MiningOutcome {
    // TODO: use MiningOutcome to cause an effect
    Damaged,
    Destroyed,
    Gained(MiningDrop),
    Unbreakable,
}

#[derive(Clone)]
pub enum Block {
    Wall,
    Rock {
        health: usize,
    },
    Ore {
        health: usize,
        mining_drop: MiningDrop,
    },
    Crystal {
        mining_drop: MiningDrop,
    },
    Chest {
        mining_drop: MiningDrop,
    },
}

impl Block {
    pub fn mine(&mut self, mining_power: usize) -> MiningOutcome {
        match self {
            Block::Wall => mine_wall(),
            Block::Rock { health, .. } => mine_rock(health, mining_power),
            Block::Ore { health, .. } => mine_ore(health, mining_power),
            Block::Crystal { .. } => mine_crystal(),
            Block::Chest { .. } => mine_chest(),
        }
    }
    pub fn colour(&self) -> Color {
        match self {
            Block::Wall => DARKGRAY,
            Block::Rock { health } => colour_map(*health),
            Block::Ore { .. } => GOLD,
            Block::Crystal { .. } => RED,
            Block::Chest { .. } => DARKBROWN,
        }
    }
}

fn colour_map(health: usize) -> Color {
    match health {
        1 => Color::new(0.0, 0.0, 0.9, 1.0),
        2 => Color::new(0.0, 0.0, 0.8, 1.0),
        3 => Color::new(0.0, 0.0, 0.7, 1.0),
        4 => Color::new(0.0, 0.0, 0.6, 1.0),
        5 => Color::new(0.0, 0.0, 0.5, 1.0),
        6 => Color::new(0.0, 0.0, 0.4, 1.0),
        7 => Color::new(0.0, 0.0, 0.3, 1.0),
        8 => Color::new(0.0, 0.0, 0.2, 1.0),
        9 => Color::new(0.0, 0.0, 0.1, 1.0),
        _ => BLACK,
    }
}

// per block type mining functions
fn mine_rock(health: &mut usize, mining_power: usize) -> MiningOutcome {
    *health = health.saturating_sub(mining_power);
    if *health == 0 {
        return MiningOutcome::Destroyed;
    };
    MiningOutcome::Damaged
}
fn mine_ore(health: &mut usize, mining_power: usize) -> MiningOutcome {
    *health = health.saturating_sub(mining_power);
    if *health == 0 {
        return MiningOutcome::Gained(MiningDrop::Gold);
    };
    MiningOutcome::Damaged
}
fn mine_crystal() -> MiningOutcome {
    MiningOutcome::Gained(MiningDrop::Energy(10))
}
fn mine_chest() -> MiningOutcome {
    MiningOutcome::Gained(MiningDrop::Item)
}
fn mine_wall() -> MiningOutcome {
    MiningOutcome::Unbreakable
}
