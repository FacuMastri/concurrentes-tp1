pub struct Container {
    current_amount: u64,
    amount_used: u64,
}

impl Container {
    pub fn new(initial_amount: u64) -> Self {
        Self {
            current_amount: initial_amount,
            amount_used: 0,
        }
    }

    pub fn add(&mut self, amount: &u64) {
        self.current_amount += amount;
    }

    pub fn subtract(&mut self, amount: &u64) {
        self.current_amount -= amount;
        self.amount_used += amount;
    }

    pub fn has_enough(&self, amount: &u64) -> bool {
        self.current_amount >= *amount
    }

    pub fn has_any(&self) -> bool {
        self.current_amount > 0
    }

    pub fn get_current_amount(&self) -> &u64 {
        &self.current_amount
    }

    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }
}
