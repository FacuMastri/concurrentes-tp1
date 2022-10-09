pub struct CoffeeBeansToGrindContainer {
    beans: u64,
    amount_used: u64,
}

impl CoffeeBeansToGrindContainer {
    pub fn new(beans: u64) -> Self {
        Self {
            beans,
            amount_used: 0,
        }
    }

    pub fn grind(&mut self, amount: &u64) {
        self.beans -= amount;
        self.amount_used += amount;
    }

    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }

    pub fn get_beans(&self) -> &u64 {
        &self.beans
    }
}

pub struct GroundCoffeeBeansContainer {
    beans: u64,
    amount_used: u64,
}

impl GroundCoffeeBeansContainer {
    pub fn new(beans: u64) -> Self {
        Self {
            beans,
            amount_used: 0,
        }
    }

    pub fn add(&mut self, amount: &u64) {
        self.beans += amount;
    }

    pub fn subtract(&mut self, amount: &u64) {
        self.beans -= amount;
        self.amount_used += amount;
    }

    pub fn has_enough(&self, amount: &u64) -> bool {
        self.beans >= *amount
    }

    pub fn has_any(&self) -> bool {
        self.beans > 0
    }

    pub fn get_coffee_beans(&self) -> &u64 {
        &self.beans
    }

    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }
}
