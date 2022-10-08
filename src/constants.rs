pub const MAX_DISPENSERS: u64 = 3;

pub const INITIAL_MILK_FOAM: u64 = 200;
pub const INITIAL_GROUND_COFFEE_BEANS: u64 = 200;

pub const RESOURCE_ALERT_FACTOR: f64 = 0.2;
pub const COFFEE_BEANS_ALERT_THRESHOLD: f64 =
    INITIAL_GROUND_COFFEE_BEANS as f64 * RESOURCE_ALERT_FACTOR;
pub const MILK_FOAM_ALERT_THRESHOLD: f64 = INITIAL_MILK_FOAM as f64 * RESOURCE_ALERT_FACTOR;

pub const INITIAL_COFFEE_BEANS_TO_GRIND: u64 = 100000;
pub const INITIAL_COLD_MILK: u64 = 100000;

pub const BASE_TIME_RESOURCE_APPLICATION: u64 = 1000;