mod blocking_queue;
mod coffee_machine;
mod constants;
mod container;
mod order;
mod utils;

use crate::coffee_machine::CoffeeMachine;
use crate::constants::{
    BASE_TIME_RESOURCE_REFILL, COFFEE_BEANS_ALERT_THRESHOLD, MILK_FOAM_ALERT_THRESHOLD,
};
use crate::order::Order;
use blocking_queue::BlockingQueue;
use constants::{
    BASE_TIME_RESOURCE_APPLICATION, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK,
    INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, ORDER_TIME_INTERVAL_ARRIVAL,
    RESOURCE_ALERT_FACTOR, STATS_UPDATE_INTERVAL,
};

fn main() {
    // TODO ver si se puede testear algo
    // TODO ver una cosa de las hipotesis
    // TODO agregar las docs de structs
    let coffee_machine = CoffeeMachine::new();
    coffee_machine.start();
}
