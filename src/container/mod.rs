/// Represents a container with a certain current amount and amount used.
pub struct Container {
    current_amount: u64,
    amount_used: u64,
}

impl Container {
    /// Creates new container with initial amount and amount used.
    pub fn new(initial_amount: u64) -> Self {
        Self {
            current_amount: initial_amount,
            amount_used: 0,
        }
    }

    /// Adds amount to the current amount of the container.
    pub fn add(&mut self, amount: &u64) {
        self.current_amount += amount;
    }

    /// Removes amount from the current amount of the container.
    /// Also, adds the amount to the amount used.
    pub fn subtract(&mut self, amount: &u64) {
        self.current_amount -= amount;
        self.amount_used += amount;
    }

    /// Returns if the container has the current amount.
    pub fn has_enough(&self, amount: &u64) -> bool {
        self.current_amount >= *amount
    }

    /// Returns if the container has any amount.
    pub fn has_any(&self) -> bool {
        self.current_amount > 0
    }

    /// Returns the current amount of the container.
    pub fn get_current_amount(&self) -> &u64 {
        &self.current_amount
    }

    /// Returns the amount used of the container.
    pub fn get_amount_used(&self) -> &u64 {
        &self.amount_used
    }
}
