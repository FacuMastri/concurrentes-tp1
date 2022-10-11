use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Order {
    coffee: u64,
    milk: u64,
    water: u64,
}

impl Order {
    pub fn new(coffee: u64, milk: u64, water: u64) -> Self {
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
