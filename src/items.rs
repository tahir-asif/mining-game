pub type ItemId = u8;

pub struct Backpack {
    pub id: ItemId,
    pub name: &'static str,
    pub starting_tech: u16,
    pub price: u16,
}

impl Backpack {
    pub fn get_tech(&self) -> u16 {
        self.starting_tech
    }
}

pub struct Hat {
    pub id: ItemId,
    pub name: &'static str,
    pub starting_energy: u16,
    pub price: u16,
}

impl Hat {
    pub fn get_energy(&self) -> u16 {
        self.starting_energy
    }
}

pub struct Pickaxe {
    pub id: ItemId,
    pub name: &'static str,
    pub mining_power: u16,
    pub price: u16,
}

impl Pickaxe {
    pub fn get_mining_power(&self) -> u16 {
        self.mining_power
    }
}

// ----------------------------------------------------------------------------
// All Items
// ----------------------------------------------------------------------------

pub static STARTING_PICK: Pickaxe = Pickaxe {
    id: 0,
    name: "Starting Pick",
    mining_power: 1,
    price: 0,
};

pub static STARTING_HAT: Hat = Hat {
    id: 0,
    name: "Starting Hat",
    starting_energy: 10,
    price: 0,
};

pub static STARTING_BACKPACK: Backpack = Backpack {
    id: 0,
    name: "Starting Backpack",
    starting_tech: 10,
    price: 0,
};

pub static IRON_PICK: Pickaxe = Pickaxe {
    id: 0,
    name: "Iron Pick",
    mining_power: 2,
    price: 10,
};

pub static GOLD_PICK: Pickaxe = Pickaxe {
    id: 0,
    name: "Gold Pick",
    mining_power: 5,
    price: 20,
};

pub static DIAMOND_PICK: Pickaxe = Pickaxe {
    id: 0,
    name: "Diamond Pick",
    mining_power: 50,
    price: 100,
};
