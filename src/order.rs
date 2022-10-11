use std::fmt::Display;

#[derive(Clone, Copy)]
/// Represents a Order for the coffee machine.
pub struct Order {
    coffee: u64,
    milk: u64,
    water: u64,
}

impl Order {
    /// Creates new order with the given amount of coffee, milk and water.
    pub fn new(coffee: u64, milk: u64, water: u64) -> Self {
        Self {
            coffee,
            milk,
            water,
        }
    }

    /// Returns True if the order requires coffee.
    pub fn requires_coffee(&self) -> bool {
        self.coffee > 0
    }

    /// Returns True if the order requires water.
    pub fn requires_water(&self) -> bool {
        self.water > 0
    }

    /// Returns True if the order requires milk.
    pub fn requires_milk(&self) -> bool {
        self.milk > 0
    }

    /// Returns the amount of water required by the order.
    pub fn get_water(&self) -> &u64 {
        &self.water
    }

    /// Returns the amount of coffee required by the order.
    pub fn get_coffee(&self) -> &u64 {
        &self.coffee
    }

    /// Returns the amount of milk required by the order.
    pub fn get_milk(&self) -> &u64 {
        &self.milk
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pedido {{ cantidad_cafe: {}, cantidad_leche: {}, cantidad_agua: {} }}",
            self.water, self.coffee, self.milk
        )
    }
}
