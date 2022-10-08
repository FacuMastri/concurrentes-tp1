struct Order {
    water: u64,
    coffee: u64,
    milk: u64,
}

impl Order {
    pub fn new(water: u64, coffee: u64, milk: u64) -> Self {
        Self { water, coffee, milk }
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
}