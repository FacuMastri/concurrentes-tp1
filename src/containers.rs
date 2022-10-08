pub struct CoffeeBeansToGrindContainer {
    beans: u64,
    amount_used: u64,
}

impl CoffeeBeansToGrindContainer {
    pub fn new(beans: u64) -> Self {
        Self { beans, amount_used: 0 }
    }

    pub fn grind(&mut self, amount: &u64) {
        self.beans -= amount;
        self.amount_used += amount;
    }

    fn increase_amount_used(&mut self, amount: u64) {
        self.amount_used += amount;
    }
}

pub struct GroundCoffeeBeansContainer {
    beans: u64,
    amount_used: u64,
}

impl GroundCoffeeBeansContainer {
    pub fn new(beans: u64) -> Self {
        Self { beans, amount_used: 0 }
    }

    pub fn add(&mut self, amount: u64) {
        self.beans += amount;
    }

    pub fn subtract(&mut self, amount: u64) {
        self.beans -= amount;
    }

    pub fn increase_amount_used(&mut self, amount: u64) {
        self.amount_used += amount;
    }
}

pub struct ColdMilkContainer {
    milk: u64,
    amount_used: u64,
}

impl ColdMilkContainer {
    pub fn new(milk: u64) -> Self {
        Self { milk, amount_used: 0 }
    }

    pub fn add(&mut self, amount: u64) {
        self.milk += amount;
    }

    pub fn subtract(&mut self, amount: u64) {
        self.milk -= amount;
    }

    pub fn increase_amount_used(&mut self, amount: u64) {
        self.amount_used += amount;
    }
}

pub struct MilkFoamContainer {
    milk: u64,
    amount_used: u64,
}

impl MilkFoamContainer {
    pub fn new(milk: u64) -> Self {
        Self { milk, amount_used: 0 }
    }

    pub fn add(&mut self, amount: u64) {
        self.milk += amount;
    }

    pub fn subtract(&mut self, amount: u64) {
        self.milk -= amount;
    }

    pub fn increase_amount_used(&mut self, amount: u64) {
        self.amount_used += amount;
    }
}
