use crate::{
    items::{Backpack, Hat, ItemId, Pickaxe},
    level::winnings::Winnings,
};

pub struct Equipped {
    pub pickaxe: &'static Pickaxe,
    pub hat: &'static Hat,
    pub backpack: &'static Backpack,
}

pub struct Profile {
    pub name: String,
    pub slot: usize,
    pub gold: u16,
    pub items: Vec<ItemId>,
    pub equipped: Equipped,
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
