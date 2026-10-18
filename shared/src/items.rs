pub type ItemId = u8;

pub struct Backpack {
    pub name: &'static str,
    pub starting_tech: u16,
    pub price: u16,
}

pub struct Hat {
    pub name: &'static str,
    pub starting_energy: u16,
    pub price: u16,
}

pub struct Pickaxe {
    pub name: &'static str,
    pub price: u16,
    pub mining_power: u16,
}

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

impl ItemRef {
    pub fn name(self) -> &'static str {
        match self {
            ItemRef::Pickaxe(item) => item.name,
            ItemRef::Hat(item) => item.name,
            ItemRef::Backpack(item) => item.name,
        }
    }

    pub fn kind(self) -> ItemKind {
        match self {
            ItemRef::Pickaxe(_) => ItemKind::Pickaxe,
            ItemRef::Hat(_) => ItemKind::Hat,
            ItemRef::Backpack(_) => ItemKind::Backpack,
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

    pub fn price(self) -> u16 {
        match self {
            ItemRef::Pickaxe(pickaxe) => pickaxe.price,
            ItemRef::Hat(hat) => hat.price,
            ItemRef::Backpack(backpack) => backpack.price,
        }
    }

    pub fn same_item(self, other: ItemRef) -> bool {
        match (self, other) {
            (ItemRef::Pickaxe(a), ItemRef::Pickaxe(b)) => std::ptr::eq(a, b),
            (ItemRef::Hat(a), ItemRef::Hat(b)) => std::ptr::eq(a, b),
            (ItemRef::Backpack(a), ItemRef::Backpack(b)) => std::ptr::eq(a, b),
            _ => false,
        }
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

pub struct ItemEntry {
    pub id: ItemId,
    pub item: ItemRef,
}

#[rustfmt::skip]
pub const ITEMS: &[ItemEntry] = &[
    ItemEntry { id: 0, item: ItemRef::Pickaxe(&STARTING_PICK) },
    ItemEntry { id: 1, item: ItemRef::Hat(&STARTING_HAT) },
    ItemEntry { id: 2, item: ItemRef::Backpack(&STARTING_BACKPACK) },
    ItemEntry { id: 3, item: ItemRef::Pickaxe(&IRON_PICK) },
    ItemEntry { id: 4, item: ItemRef::Pickaxe(&GOLD_PICK) },
    ItemEntry { id: 5, item: ItemRef::Pickaxe(&DIAMOND_PICK) },
];

pub fn get_item(id: ItemId) -> Option<ItemRef> {
    ITEMS
        .iter()
        .find(|entry| entry.id == id)
        .map(|entry| entry.item)
}

// Definintions below

pub static STARTING_PICK: Pickaxe = Pickaxe {
    name: "Starting Pick",
    price: 1,
    mining_power: 1,
};
pub static STARTING_HAT: Hat = Hat {
    name: "Starting Hat",
    price: 1,
    starting_energy: 10,
};
pub static STARTING_BACKPACK: Backpack = Backpack {
    name: "Starting Backpack",
    price: 1,
    starting_tech: 10,
};
pub static IRON_PICK: Pickaxe = Pickaxe {
    name: "Iron Pick",
    price: 10,
    mining_power: 2,
};
pub static GOLD_PICK: Pickaxe = Pickaxe {
    name: "Gold Pick",
    price: 30,
    mining_power: 5,
};
pub static DIAMOND_PICK: Pickaxe = Pickaxe {
    name: "Diamond Pick",
    price: 100,
    mining_power: 50,
};
