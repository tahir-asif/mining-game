use crate::level::winnings::Winnings;
use shared::items::{Backpack, Hat, ItemRef, Pickaxe};

pub struct Equipped {
    pub pickaxe: &'static Pickaxe,
    pub hat: &'static Hat,
    pub backpack: &'static Backpack,
}

pub struct Profile {
    pub name: String,
    pub slot: usize,
    pub gold: u16,
    pub items: Vec<ItemRef>,
    pub equipped: Equipped,
}

impl Profile {
    pub fn starting_energy(&self) -> u16 {
        self.equipped.hat.starting_energy
    }

    pub fn starting_tech(&self) -> u16 {
        self.equipped.backpack.starting_tech
    }

    pub fn get_mining_power(&self) -> u16 {
        self.equipped.pickaxe.mining_power
    }

    pub fn collect_winnings(&mut self, winnings: Winnings) {
        self.gold += winnings.gold;
        self.items.extend(winnings.items);
    }
}
