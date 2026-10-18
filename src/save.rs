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

    pub fn equip(&mut self, item: ItemRef) {
        self.remove_item(item);

        match item {
            ItemRef::Pickaxe(pickaxe) => {
                self.items.push(ItemRef::Pickaxe(self.equipped.pickaxe));
                self.equipped.pickaxe = pickaxe;
            }
            ItemRef::Hat(hat) => {
                self.items.push(ItemRef::Hat(self.equipped.hat));
                self.equipped.hat = hat;
            }
            ItemRef::Backpack(backpack) => {
                self.items.push(ItemRef::Backpack(self.equipped.backpack));
                self.equipped.backpack = backpack;
            }
        }
    }

    pub fn sell(&mut self, item: ItemRef) {
        self.remove_item(item);

        self.gold += item.price();
    }

    fn remove_item(&mut self, item: ItemRef) {
        if let Some(index) = self.items.iter().position(|&i| i.same_item(item)) {
            self.items.remove(index);
        }
    }

    pub fn collect_winnings(&mut self, winnings: Winnings) {
        self.gold += winnings.gold;
        self.items.extend(winnings.items);
    }
}
