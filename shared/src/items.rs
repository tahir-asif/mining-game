use std::ops::Range;

pub type ItemId = u8;

pub struct Backpack {
    pub id: ItemId,
    pub name: &'static str,
    pub starting_tech: u16,
    pub price: u16,
}

pub struct Hat {
    pub id: ItemId,
    pub name: &'static str,
    pub starting_energy: u16,
    pub price: u16,
}

pub struct Pickaxe {
    pub id: ItemId,
    pub name: &'static str,
    pub price: u16,
    pub mining_power: u16,
}

#[derive(PartialEq, Debug)]
pub enum ItemKind {
    Backpack,
    Hat,
    Pickaxe,
}

#[derive(Clone, Copy)]
pub enum ItemRef {
    Pickaxe(&'static Pickaxe),
    Hat(&'static Hat),
    Backpack(&'static Backpack),
}

impl PartialEq for ItemRef {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for ItemRef {}

impl ItemRef {
    pub const fn kind(self) -> ItemKind {
        match self {
            ItemRef::Pickaxe(_) => ItemKind::Pickaxe,
            ItemRef::Hat(_) => ItemKind::Hat,
            ItemRef::Backpack(_) => ItemKind::Backpack,
        }
    }

    pub const fn id(self) -> ItemId {
        match self {
            ItemRef::Pickaxe(item) => item.id,
            ItemRef::Hat(item) => item.id,
            ItemRef::Backpack(item) => item.id,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            ItemRef::Pickaxe(item) => item.name,
            ItemRef::Hat(item) => item.name,
            ItemRef::Backpack(item) => item.name,
        }
    }

    pub const fn price(self) -> u16 {
        match self {
            ItemRef::Pickaxe(pickaxe) => pickaxe.price,
            ItemRef::Hat(hat) => hat.price,
            ItemRef::Backpack(backpack) => backpack.price,
        }
    }

    pub fn is_pickaxe(self) -> bool {
        matches!(self, ItemRef::Pickaxe(_))
    }

    pub fn is_hat(self) -> bool {
        matches!(self, ItemRef::Hat(_))
    }

    pub fn is_backpack(self) -> bool {
        matches!(self, ItemRef::Backpack(_))
    }

    pub fn pickaxe(self) -> Option<&'static Pickaxe> {
        match self {
            ItemRef::Pickaxe(item) => Some(item),
            _ => None,
        }
    }

    pub fn hat(self) -> Option<&'static Hat> {
        match self {
            ItemRef::Hat(item) => Some(item),
            _ => None,
        }
    }

    pub fn backpack(self) -> Option<&'static Backpack> {
        match self {
            ItemRef::Backpack(item) => Some(item),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------
// All Items
// ----------------------------------------------------------------------------

pub const STARTING_PICKAXE_ID: ItemId = 0;

pub const PICKAXE_COUNT: u8 = 4;
pub const HAT_COUNT: u8 = 1;
pub const BACKPACK_COUNT: u8 = 1;

pub const PICKAXE_RANGE: Range<ItemId> = 0..PICKAXE_COUNT;
pub const HAT_RANGE: Range<ItemId> = PICKAXE_COUNT..(PICKAXE_COUNT + HAT_COUNT);
pub const BACKPACK_RANGE: Range<ItemId> =
    (PICKAXE_COUNT + HAT_COUNT)..(PICKAXE_COUNT + HAT_COUNT + BACKPACK_COUNT);

#[rustfmt::skip]
pub const ITEMS: &[ItemRef] = &[
    ItemRef::Pickaxe(&STARTING_PICKAXE),
    ItemRef::Pickaxe(&IRON_PICK),
    ItemRef::Pickaxe(&GOLD_PICK),
    ItemRef::Pickaxe(&DIAMOND_PICK),
    ItemRef::Hat(&STARTING_HAT),
    ItemRef::Backpack(&STARTING_BACKPACK),
];

#[inline]
pub fn get_item(id: ItemId) -> Option<ItemRef> {
    ITEMS.get(id as usize).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_ids_are_positional_and_category_ordered_and_unique() {
        let mut seen = std::collections::HashSet::new();
        for (i, item) in ITEMS.iter().enumerate() {
            assert_eq!(item.id() as usize, i, "ITEMS index != id at {i}");
            assert!(seen.insert(item.id()), "duplicate id {}", item.id());
            let expected = if i < PICKAXE_COUNT as usize {
                ItemKind::Pickaxe
            } else if i < (PICKAXE_COUNT + HAT_COUNT) as usize {
                ItemKind::Hat
            } else {
                ItemKind::Backpack
            };
            assert_eq!(item.kind(), expected, "wrong category at index {i}");
        }
        assert_eq!(
            ITEMS.len(),
            (PICKAXE_COUNT + HAT_COUNT + BACKPACK_COUNT) as usize
        );
    }

    #[test]
    fn count_consts_are_correct() {
        let mut pickaxe_count = 0;
        let mut hat_count = 0;
        let mut backpack_count = 0;
        for item in ITEMS.iter() {
            match *item {
                ItemRef::Pickaxe(_) => pickaxe_count += 1,
                ItemRef::Hat(_) => hat_count += 1,
                ItemRef::Backpack(_) => backpack_count += 1,
            }
        }
        assert_eq!(pickaxe_count, PICKAXE_COUNT, "PICKAXE_COUNT is incorrect");
        assert_eq!(hat_count, HAT_COUNT, "HAT_COUNT is incorrect");
        assert_eq!(
            backpack_count, BACKPACK_COUNT,
            "BACKPACK_COUNT is incorrect"
        );
    }
}

// Definintions below

pub static STARTING_PICKAXE: Pickaxe = Pickaxe {
    id: 0,
    name: "Starting Pick",
    price: 1,
    mining_power: 1,
};
pub static IRON_PICK: Pickaxe = Pickaxe {
    id: 1,
    name: "Iron Pick",
    price: 10,
    mining_power: 2,
};
pub static GOLD_PICK: Pickaxe = Pickaxe {
    id: 2,
    name: "Gold Pick",
    price: 30,
    mining_power: 5,
};
pub static DIAMOND_PICK: Pickaxe = Pickaxe {
    id: 3,
    name: "Diamond Pick",
    price: 100,
    mining_power: 50,
};
pub static STARTING_HAT: Hat = Hat {
    id: 4,
    name: "Starting Hat",
    price: 1,
    starting_energy: 10,
};
pub static STARTING_BACKPACK: Backpack = Backpack {
    id: 5,
    name: "Starting Backpack",
    price: 1,
    starting_tech: 10,
};
