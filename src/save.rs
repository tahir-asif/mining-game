use crate::{
    items::{Backpack, Hat, ItemId, Pickaxe},
    level_session::winnings::Winnings,
};

pub struct Equipment {
    pub pickaxe: &'static Pickaxe,
    pub hat: &'static Hat,
    pub backpack: &'static Backpack,
}

pub struct Profile {
    pub name: String,
    pub slot: usize,
    pub gold: u16,
    pub items: Vec<ItemId>,
    pub equipped: Equipment,
}

impl Profile {
    pub fn starting_energy(&self) -> u16 {
        self.equipped.hat.get_energy()
    }

    pub fn starting_tech(&self) -> u16 {
        self.equipped.backpack.get_tech()
    }

    pub fn get_mining_power(&self) -> u16 {
        self.equipped.pickaxe.get_mining_power()
    }

    pub fn collect_winnings(&mut self, winnings: Winnings) {
        self.gold += winnings.gold;
    }
}

pub static STARTING_PICK: Pickaxe = Pickaxe {
    id: 0,
    name: "Starting Pick",
    mining_power: 1,
    price: 0,
};

pub static STARTING_HAT: Hat = Hat {
    id: 0,
    name: "Starting Pick",
    starting_energy: 10,
    price: 0,
};

pub static STARTING_BACKPACK: Backpack = Backpack {
    id: 0,
    name: "Starting Pick",
    starting_tech: 10,
    price: 0,
};
