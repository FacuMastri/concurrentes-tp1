struct CoffeeBeansToGrindContainer {
    beans: u64,
    amount_used: u64,
}

impl CoffeeBeansToGrindContainer {
    pub fn new(beans: u64, amount_used: u64) -> Self {
        Self { beans, amount_used }
    }

    pub fn grind(&mut self, amount: u64) {
        self.beans -= amount;
    }

    pub fn increase_amount_used(&mut self, amount: u64) {
        self.amount_used += amount;
    }

}

struct GroundCoffeeBeansContainer {
    beans: u64,
    amount_used: u64
}

impl GroundCoffeeBeansContainer {
    pub fn new(beans: u64, amount_used: u64) -> Self {
        Self { beans, amount_used }
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

struct ColdMilkContainer {
    milk: u64,
    amount_used: u64
}

impl ColdMilkContainer {
    pub fn new(milk: u64, amount_used: u64) -> Self {
        Self { milk, amount_used }
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

struct MilkFoamContainer {
    milk: u64,
    amount_used: u64
}

impl MilkFoamContainer {
    pub fn new(milk: u64, amount_used: u64) -> Self {
        Self { milk, amount_used }
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