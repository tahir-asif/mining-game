use crate::level::winnings::Winnings;
use shared::{
    ItemId,
    items::{Backpack, Hat, ItemRef, Pickaxe},
};
use std::cmp;

pub struct Equipped {
    pub pickaxe: &'static Pickaxe,
    pub hat: &'static Hat,
    pub backpack: &'static Backpack,
}

pub struct Profile {
    pub name: String,
    pub save_slot: u8,
    pub gold: u16,
    items: Vec<ItemId>,
    pub equipped: Equipped,
    pub latest_completed_level: u8,
}

impl Profile {
    pub const fn new(name: String, save_slot: u8) -> Self {
        Profile {
            name,
            save_slot,
            gold: 0,
            items: vec![],
            equipped: Equipped {
                pickaxe: &shared::STARTING_PICKAXE,
                hat: &shared::STARTING_HAT,
                backpack: &shared::STARTING_BACKPACK,
            },
            latest_completed_level: 0,
        }
    }

    pub fn starting_energy(&self) -> u16 {
        self.equipped.hat.starting_energy
    }

    pub fn starting_tech(&self) -> u16 {
        self.equipped.backpack.starting_tech
    }

    pub fn get_mining_power(&self) -> u16 {
        self.equipped.pickaxe.mining_power
    }

    pub fn equip(&mut self, item: ItemRef) {
        self.remove_item(item.id());

        match item {
            ItemRef::Pickaxe(pickaxe) => {
                self.add_item(ItemRef::Pickaxe(self.equipped.pickaxe).id());
                self.equipped.pickaxe = pickaxe;
            }
            ItemRef::Hat(hat) => {
                self.add_item(ItemRef::Hat(self.equipped.hat).id());
                self.equipped.hat = hat;
            }
            ItemRef::Backpack(backpack) => {
                self.add_item(ItemRef::Backpack(self.equipped.backpack).id());
                self.equipped.backpack = backpack;
            }
        }
    }

    pub fn sell_item(&mut self, item: ItemRef) {
        self.remove_item(item.id());
        self.gold += item.price();
    }

    pub fn items(&self) -> &[ItemId] {
        &self.items
    }

    pub fn add_item(&mut self, id: ItemId) {
        let i = self.items.partition_point(|&x| x < id);
        self.items.insert(i, id);
    }

    pub fn remove_item(&mut self, id: ItemId) {
        let i = self.items.partition_point(|&x| x < id);
        if self.items.get(i) == Some(&id) {
            self.items.remove(i);
        }
    }

    pub fn collect_winnings(&mut self, winnings: Winnings) {
        self.gold += winnings.gold;
        for item in winnings.items {
            self.add_item(item.id());
        }
        self.latest_completed_level =
            cmp::max(winnings.latest_completed_level, self.latest_completed_level);
    }
}
