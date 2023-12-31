use crate::Order;

#[derive(Clone, Copy, Debug)]
/// Represents the amount of resources in the coffee machine.
pub enum Resource {
    Coffee = 0,
    Milk,
    Water,
}

/// Useful for the coffee machine to know if an order has arrived or if it should shutdown.
pub enum Message {
    Job(Order),
    Shutdown,
}

pub mod converter {
    use crate::constants::{COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET};
    use crate::container::Container;
    use crate::BASE_TIME_RESOURCE_REFILL;
    use std::sync::MutexGuard;
    use std::thread;
    use std::time::Duration;

    /// Refills the given container with the given amount from the given container.
    fn refill_container(
        from_container: &mut MutexGuard<Container>,
        value_to_refill: &u64,
        to_container: &mut MutexGuard<Container>,
    ) {
        thread::sleep(Duration::from_millis(
            BASE_TIME_RESOURCE_REFILL * value_to_refill,
        ));
        from_container.subtract(value_to_refill);
        to_container.add(value_to_refill);
    }

    /// Refills the given coffee container with the given amount from coffee beans container.
    pub fn refill_coffee(
        ground_coffee_beans_container: &mut MutexGuard<Container>,
        value_to_refill: &u64,
        mut coffee_beans_to_grind_container: MutexGuard<Container>,
    ) {
        println!(
            "{}[Refill de café]{} - Convirtiendo {} de granos para moler a granos molidos",
            COLOR_CYAN, COLOR_RESET, value_to_refill
        );
        refill_container(
            &mut coffee_beans_to_grind_container,
            value_to_refill,
            ground_coffee_beans_container,
        );
        println!(
            "{}[Refill de café]{} - Terminó de convertir granos de café",
            COLOR_CYAN, COLOR_RESET
        );
    }

    /// Refills the given milk container with the given amount from cold milk container.
    pub fn refill_milk(
        milk_foam_container: &mut MutexGuard<Container>,
        value_to_refill: &u64,
        mut cold_milk_container: MutexGuard<Container>,
    ) {
        println!(
            "{}[Refill de leche espumada]{} - Convirtiendo {} de leche a leche espumada",
            COLOR_MAGENTA, COLOR_RESET, value_to_refill
        );
        refill_container(
            &mut cold_milk_container,
            value_to_refill,
            milk_foam_container,
        );
        println!(
            "{}[Refill de leche espumada]{} - Terminó de convertir leche espumada",
            COLOR_MAGENTA, COLOR_RESET
        );
    }
}
