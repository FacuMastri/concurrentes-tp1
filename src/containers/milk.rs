pub struct ColdMilkContainer {
    milk: u64,
    amount_used: u64,
}

impl ColdMilkContainer {
    pub fn new(milk: u64) -> Self {
        Self {
            milk,
            amount_used: 0,
        }
    }

    pub fn subtract(&mut self, amount: &u64) {
        self.milk -= amount;
        self.amount_used += amount;
    }

    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }

    pub fn get_milk(&self) -> &u64 {
        &self.milk
    }
}

pub struct MilkFoamContainer {
    milk: u64,
    amount_used: u64,
}

impl MilkFoamContainer {
    pub fn new(milk: u64) -> Self {
        Self {
            milk,
            amount_used: 0,
        }
    }

    pub fn add(&mut self, amount: &u64) {
        self.milk += amount;
    }

    pub fn subtract(&mut self, amount: &u64) {
        self.milk -= amount;
        self.amount_used += amount;
    }

    pub fn has_enough(&self, amount: &u64) -> bool {
        self.milk >= *amount
    }

    pub fn has_any(&self) -> bool {
        self.milk > 0
    }

    pub fn get_milk(&self) -> &u64 {
        &self.milk
    }

    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }
}
