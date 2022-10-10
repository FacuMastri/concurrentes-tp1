/// Maximum amount of dispensers a coffee machine can have
pub const MAX_DISPENSERS: u64 = 3;

/// Maximum amount (capacity) of a milk foam container
pub const INITIAL_MILK_FOAM: u64 = 100;
/// Maximum amount (capacity) of a ground coffee beans container
pub const INITIAL_GROUND_COFFEE_BEANS: u64 = 100;

/// Custom resource alert factor
pub const RESOURCE_ALERT_FACTOR: f64 = 0.2;
/// Coffee beans alert threshold, used to trigger a console alert when the amount of coffee beans is below this threshold
pub const COFFEE_BEANS_ALERT_THRESHOLD: f64 =
    INITIAL_GROUND_COFFEE_BEANS as f64 * RESOURCE_ALERT_FACTOR;
/// Milk foam alert threshold, used to trigger a console alert when the amount of milk foam is below this threshold
pub const MILK_FOAM_ALERT_THRESHOLD: f64 = INITIAL_MILK_FOAM as f64 * RESOURCE_ALERT_FACTOR;

/// Maximum amount (capacity) of a coffee beans to grind container
pub const INITIAL_COFFEE_BEANS_TO_GRIND: u64 = 100000;
/// Maximum amount (capacity) of a cold milk container
pub const INITIAL_COLD_MILK: u64 = 100000;

/// Time (in milliseconds) to apply a resource
pub const BASE_TIME_RESOURCE_APPLICATION: u64 = 500;
/// Time (in milliseconds) to refill a resource
pub const BASE_TIME_RESOURCE_REFILL: u64 = (0.10 * BASE_TIME_RESOURCE_APPLICATION as f64) as u64;

/// Time (in seconds) between each coffee machine status update
pub const STATS_UPDATE_TIME: u64 = 5;

/// Time (in milliseconds) between each order taken. This is used to simulate the arrival of a customer.
pub const ORDER_TIME_ARRIVAL: u64 = 2000;
