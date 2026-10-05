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
