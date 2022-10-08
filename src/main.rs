mod constants;
mod containers;
mod order;

use crate::constants::{COFFEE_BEANS_ALERT_THRESHOLD, MILK_FOAM_ALERT_THRESHOLD};
use crate::containers::{
    CoffeeBeansToGrindContainer, ColdMilkContainer, GroundCoffeeBeansContainer, MilkFoamContainer,
};
use constants::{
    BASE_TIME_RESOURCE_APPLICATION, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK,
    INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, RESOURCE_ALERT_FACTOR,
};
use rand::prelude::*;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let coffee_beans_to_grind_container = Arc::new(Mutex::new(CoffeeBeansToGrindContainer::new(
        INITIAL_COFFEE_BEANS_TO_GRIND,
    )));
    let cold_milk_container = Arc::new(Mutex::new(ColdMilkContainer::new(INITIAL_COLD_MILK)));

    let ground_coffee_beans_container = Arc::new((
        Mutex::new(GroundCoffeeBeansContainer::new(INITIAL_GROUND_COFFEE_BEANS)),
        Condvar::new(),
    ));
    let milk_foam_container = Arc::new((
        Mutex::new(MilkFoamContainer::new(INITIAL_MILK_FOAM)),
        Condvar::new(),
    ));

    let total_drinks_prepared = Arc::new(Mutex::new(0));

    #[allow(clippy::needless_collect)]
    let dispensers: Vec<JoinHandle<()>> = (1..MAX_DISPENSERS + 1)
        .map(|i| {
            let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
            let milk_foam_clone = milk_foam_container.clone();
            let total_drinks_prepared_clone = total_drinks_prepared.clone();
            thread::spawn(move || {
                make_drink(
                    i,
                    ground_coffee_beans_clone,
                    milk_foam_clone,
                    total_drinks_prepared_clone,
                );
            })
        })
        .collect();

    let cold_milk_clone = cold_milk_container.clone();
    let milk_foam_clone = milk_foam_container.clone();

    let milk_refill = thread::spawn(move || transform_milk(cold_milk_clone, milk_foam_clone));

    let ground_coffee_beans_container_clone = ground_coffee_beans_container.clone();
    let coffee_beans_to_grind_container_clone = coffee_beans_to_grind_container.clone();
    let coffee_refill = thread::spawn(move || {
        transform_coffee(
            ground_coffee_beans_container_clone,
            coffee_beans_to_grind_container_clone,
        )
    });

    let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
    let alert_system_for_coffee_beans =
        thread::spawn(move || inform_about_coffee_beans(ground_coffee_beans_clone));

    let milk_foam_container_clone = milk_foam_container.clone();
    let alert_system_for_milk =
        thread::spawn(move || inform_about_milk_foam(milk_foam_container_clone));

    coffee_refill.join().unwrap();
    milk_refill.join().unwrap();
    alert_system_for_coffee_beans.join().unwrap();
    alert_system_for_milk.join().unwrap();

    let _: Vec<()> = dispensers
        .into_iter()
        .flat_map(|dispenser| dispenser.join())
        .collect();
}

fn inform_about_milk_foam(
    milk_foam_container_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
) -> ! {
    loop {
        let (lock, cvar) = &*milk_foam_container_clone;
        let milk_foam = cvar
            .wait_while(lock.lock().unwrap(), |milk_foam| {
                milk_foam.has_enough(&(MILK_FOAM_ALERT_THRESHOLD as u64))
            })
            .unwrap();
        println!(
            "[Alerta de recursos: leche] El nivel de leche espumada es de {} (threshold de {}%)",
            (*milk_foam).get_milk(),
            RESOURCE_ALERT_FACTOR * 100.0
        );
    }
}

fn inform_about_coffee_beans(
    ground_coffee_beans_clone: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
) -> ! {
    loop {
        let (lock, cvar) = &*ground_coffee_beans_clone;
        let ground_coffee_beans = cvar
            .wait_while(lock.lock().unwrap(), |ground_coffee_beans| {
                ground_coffee_beans.has_enough(&(COFFEE_BEANS_ALERT_THRESHOLD as u64))
            })
            .unwrap();
        println!(
            "[Alerta de recursos: café] El nivel de granos de café es de {} (threshold de {}%)",
            (*ground_coffee_beans).get_coffee_beans(),
            RESOURCE_ALERT_FACTOR * 100.0
        );
    }
}

fn transform_coffee(
    ground_coffee_beans_container_clone: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
    coffee_beans_to_grind_container_clone: Arc<Mutex<CoffeeBeansToGrindContainer>>,
) -> ! {
    loop {
        let (lock, cvar) = &*ground_coffee_beans_container_clone;
        let ground_coffee_beans = cvar
            .wait_while(lock.lock().unwrap(), |ground_coffee_beans| {
                ground_coffee_beans.has_any()
            })
            .unwrap();
        let value_to_refill = 100;
        println!(
            "[Refill de café] Convirtiendo {} de granos para moler a granos molidos",
            value_to_refill
        );
        let coffee_beans_to_grind = coffee_beans_to_grind_container_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(1000 * value_to_refill));
        convert_coffee_beans_to_ground_beans(
            ground_coffee_beans,
            &value_to_refill,
            coffee_beans_to_grind,
        );
        println!("[Refill de café] Terminó de convertir granos de café");
        cvar.notify_all();
    }
}

fn transform_milk(
    cold_milk_clone: Arc<Mutex<ColdMilkContainer>>,
    milk_foam_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
) -> ! {
    loop {
        let (lock, cvar) = &*milk_foam_clone;
        let milk_foam = cvar
            .wait_while(lock.lock().unwrap(), |milk_foam| milk_foam.has_any())
            .unwrap();
        let value_to_refill = 100;
        println!(
            "[Refill de leche espumada] Convirtiendo {} de leche a leche espumada",
            value_to_refill
        );
        let cold_milk = cold_milk_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(
            BASE_TIME_RESOURCE_APPLICATION * value_to_refill,
        ));
        convert_milk_to_foam_milk(milk_foam, &value_to_refill, cold_milk);
        println!("[Refill de leche espumada] Terminó de convertir leche espumada");
        cvar.notify_all();
    }
}

fn make_drink(
    n_dispenser: u64,
    ground_coffee_beans_clone: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
    milk_foam_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
    total_drinks_prepared_clone: Arc<Mutex<i32>>,
) {
    loop {
        let coffee_order = random::<u64>() % 5;
        let milk_order = random::<u64>() % 5;
        let water_order = random::<u64>() % 5;
        println!("[Dispenser {}] Recibió orden de café con: {} gr de café, {} gr de leche y {} gr de agua", n_dispenser, coffee_order, milk_order, water_order);
        if coffee_order > 0 {
            let (lock, cvar) = &*ground_coffee_beans_clone;
            let mut ground_coffee_beans = cvar
                .wait_while(lock.lock().unwrap(), |ground_coffee_beans| {
                    !ground_coffee_beans.has_enough(&coffee_order)
                })
                .unwrap();
            println!(
                "[Dispenser {}] Aplicando granos de café: {}",
                n_dispenser, coffee_order
            );
            thread::sleep(Duration::from_millis(
                (BASE_TIME_RESOURCE_APPLICATION * coffee_order) as u64,
            ));
            ground_coffee_beans.subtract(&coffee_order);
            println!(
                "[Dispenser {}] Terminó de aplicar granos de café",
                n_dispenser
            );
            cvar.notify_all();
        }

        if milk_order > 0 {
            let (lock, cvar) = &*milk_foam_clone;
            let mut milk_foam = cvar
                .wait_while(lock.lock().unwrap(), |milk_foam| {
                    !milk_foam.has_enough(&milk_order)
                })
                .unwrap();
            println!(
                "[Dispenser {}] Aplicando leche espumada: {}",
                n_dispenser, milk_order
            );
            thread::sleep(Duration::from_millis(
                (BASE_TIME_RESOURCE_APPLICATION * milk_order) as u64,
            ));
            milk_foam.subtract(&milk_order);
            println!("[Dispenser {}] Terminó de aplicar leche", n_dispenser);
            cvar.notify_all();
        }

        if water_order > 0 {
            println!("[Dispenser {}] Aplicando agua", n_dispenser);
            thread::sleep(Duration::from_millis(
                (BASE_TIME_RESOURCE_APPLICATION * water_order) as u64,
            ));
            println!("[Dispenser {}] Terminó de aplicar agua", n_dispenser);
        }

        println!("[Dispenser {}] Terminó de hacer café", n_dispenser);
        let mut total_drinks = total_drinks_prepared_clone.lock().unwrap();
        *total_drinks += 1;
    }
}

fn convert_coffee_beans_to_ground_beans(
    mut ground_coffee_beans: MutexGuard<GroundCoffeeBeansContainer>,
    value_to_refill: &u64,
    mut coffee_beans_to_grind: MutexGuard<CoffeeBeansToGrindContainer>,
) {
    coffee_beans_to_grind.grind(value_to_refill);
    ground_coffee_beans.add(value_to_refill);
}

fn convert_milk_to_foam_milk(
    mut milk_foam: MutexGuard<MilkFoamContainer>,
    value_to_refill: &u64,
    mut cold_milk: MutexGuard<ColdMilkContainer>,
) {
    cold_milk.subtract(value_to_refill);
    milk_foam.add(value_to_refill);
}
