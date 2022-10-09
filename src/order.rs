#[derive(Clone, Copy, Debug)]
pub struct Order {
    water: u64,
    coffee: u64,
    milk: u64,
}

impl Order {
    pub fn new(water: u64, coffee: u64, milk: u64) -> Self {
        Self {
            coffee,
            milk,
            water,
        }
    }

    pub fn requires_coffee(&self) -> bool {
        self.coffee > 0
    }

    pub fn requires_water(&self) -> bool {
        self.water > 0
    }

    pub fn requires_milk(&self) -> bool {
        self.milk > 0
    }

    pub fn get_water(&self) -> &u64 {
        &self.water
    }

    pub fn get_coffee(&self) -> &u64 {
        &self.coffee
    }

    pub fn get_milk(&self) -> &u64 {
        &self.milk
    }

    pub fn is_empty(&self) -> bool {
        self.water == 0 && self.coffee == 0 && self.milk == 0
    }
}
