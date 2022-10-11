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
/// Milk foam alert threshold, used to trigger an alert when the amount of milk foam is below this threshold
pub const MILK_FOAM_ALERT_THRESHOLD: f64 = INITIAL_MILK_FOAM as f64 * RESOURCE_ALERT_FACTOR;

/// Maximum amount (capacity) of a coffee beans to grind container. This value should be high enough to handle all possible drink order-examples in case of refill
pub const INITIAL_COFFEE_BEANS_TO_GRIND: u64 = 100000;
/// Maximum amount (capacity) of a cold milk container. This value should be high enough to handle all possible drink order-examples in case of refill
pub const INITIAL_COLD_MILK: u64 = 100000;

/// Amount of milk to refill into a milk foam container from a cold milk container when it is empty
pub const MILK_TO_REFILL: u64 = 100;
/// Amount of coffee beans to grind into a ground coffee beans container from a coffee beans container when it is empty
pub const COFFEE_TO_REFILL: u64 = 100;

/// Time (in milliseconds) to apply a resource
pub const BASE_TIME_RESOURCE_APPLICATION: u64 = 500;
/// Time (in milliseconds) to refill a resource
pub const BASE_TIME_RESOURCE_REFILL: u64 = (0.10 * BASE_TIME_RESOURCE_APPLICATION as f64) as u64;

/// Time (in seconds) between each coffee machine status update
pub const STATS_UPDATE_INTERVAL: u64 = 5;
/// Time (in milliseconds) between each order taken. This is used to simulate the arrival of a customer.
pub const ORDER_TIME_INTERVAL_ARRIVAL: u64 = 2000;
/// Time (in seconds) to wait during a **wait_timeout_while** before returning a result (if the condition is not met)
///
/// This is used to have some kind of mechanism to stop the execution of a thread when doing a graceful shutdown
pub const CONDVAR_WAIT_TIMEOUT: u64 = 5;

/// Color for the console output, according to the actor involved
pub const COLOR_RED: &str = "\x1b[31m";
/// Color for the console output, according to the actor involved
pub const COLOR_GREEN: &str = "\x1b[32m";
/// Color for the console output, according to the actor involved
pub const COLOR_YELLOW: &str = "\x1b[33m";
/// Color for the console output, according to the actor involved
pub const COLOR_BLUE: &str = "\x1b[34m";
/// Color for the console output, according to the actor involved
pub const COLOR_MAGENTA: &str = "\x1b[35m";
/// Color for the console output, according to the actor involved
pub const COLOR_CYAN: &str = "\x1b[36m";
/// Color for the console output, according to the actor involved
pub const COLOR_RESET: &str = "\x1b[0m";
