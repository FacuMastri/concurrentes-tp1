mod blocking_queue;
mod coffee_machine;
mod constants;
mod containers;
mod order;

use crate::coffee_machine::CoffeeMachine;
use crate::constants::{
    BASE_TIME_RESOURCE_REFILL, COFFEE_BEANS_ALERT_THRESHOLD, MILK_FOAM_ALERT_THRESHOLD,
};
use crate::containers::coffee_beans::{CoffeeBeansToGrindContainer, GroundCoffeeBeansContainer};
use crate::containers::milk::{ColdMilkContainer, MilkFoamContainer};
use crate::order::Order;
use blocking_queue::BlockingQueue;
use constants::{
    BASE_TIME_RESOURCE_APPLICATION, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK,
    INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, ORDER_TIME_INTERVAL_ARRIVAL,
    RESOURCE_ALERT_FACTOR, STATS_UPDATE_INTERVAL,
};

fn main() {
    // TODO printear con colores (y separar en modulo), refactorizar la lectura y algo mas, ver si se puede testear algo
    // TODO ver una cosa de las hipotesis
    let coffee_machine = CoffeeMachine::new();
    coffee_machine.start();
}
