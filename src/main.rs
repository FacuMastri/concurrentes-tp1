mod constants;
mod containers;
mod order;
mod blocking_queue;

use crate::constants::{
    BASE_TIME_RESOURCE_REFILL, COFFEE_BEANS_ALERT_THRESHOLD, MILK_FOAM_ALERT_THRESHOLD,
};
use crate::containers::{
    CoffeeBeansToGrindContainer, ColdMilkContainer, GroundCoffeeBeansContainer, MilkFoamContainer,
};
use constants::{
    BASE_TIME_RESOURCE_APPLICATION, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK,
    INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, RESOURCE_ALERT_FACTOR,
    STATS_TIME,
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
            let coffee_beans_to_grind_container_clone = coffee_beans_to_grind_container.clone();
            let cold_milk_container_clone = cold_milk_container.clone();
            thread::spawn(move || {
                make_drink(
                    i,
                    ground_coffee_beans_clone,
                    milk_foam_clone,
                    total_drinks_prepared_clone,
                    coffee_beans_to_grind_container_clone,
                    cold_milk_container_clone,
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

    let total_drinks_prepared_clone = total_drinks_prepared.clone();
    let ground_coffee_beans_container_clone = ground_coffee_beans_container.clone();
    let coffee_beans_to_grind_container_clone = coffee_beans_to_grind_container.clone();
    let cold_milk_container_clone = cold_milk_container.clone();
    let milk_foam_container_clone = milk_foam_container.clone();

    let inform_system = thread::spawn(move || {
        inform_stats(
            total_drinks_prepared_clone,
            ground_coffee_beans_container_clone,
            coffee_beans_to_grind_container_clone,
            cold_milk_container_clone,
            milk_foam_container_clone,
        )
    });

    coffee_refill.join().unwrap();
    milk_refill.join().unwrap();
    alert_system_for_coffee_beans.join().unwrap();
    alert_system_for_milk.join().unwrap();
    inform_system.join().unwrap();

    let _: Vec<()> = dispensers
        .into_iter()
        .flat_map(|dispenser| dispenser.join())
        .collect();
}

#[allow(clippy::format_push_string)]
fn inform_stats(
    total_drinks_prepared: Arc<Mutex<i32>>,
    ground_coffee_beans_container: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
    coffee_beans_to_grind_container: Arc<Mutex<CoffeeBeansToGrindContainer>>,
    cold_milk_container: Arc<Mutex<ColdMilkContainer>>,
    milk_foam_container: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
) -> ! {
    loop {
        let mut report = String::from("[Estadísticas] ");
        {
            let total_drinks = total_drinks_prepared.lock().unwrap();
            report.push_str(&format!(
                "Total de bebidas preparadas: {} || ",
                total_drinks
            ));
        }
        {
            let (lock, _cvar) = &*ground_coffee_beans_container;
            let ground_coffee_beans = lock.lock().unwrap();
            report.push_str(&format!(
                "Café molido actualmente: {} - Consumido: {} || ",
                ground_coffee_beans.get_coffee_beans(),
                ground_coffee_beans.get_amount_used()
            ));
        }
        {
            let coffee_beans_to_grind = coffee_beans_to_grind_container.lock().unwrap();
            report.push_str(&format!(
                "Café en grano actualmente: {} - Consumido: {} || ",
                coffee_beans_to_grind.get_beans(),
                coffee_beans_to_grind.get_amount_used()
            ));
        }
        {
            let cold_milk = cold_milk_container.lock().unwrap();
            report.push_str(&format!(
                "Leche fría actualmente: {} - Consumida: {} || ",
                cold_milk.get_milk(),
                cold_milk.get_amount_used()
            ));
        }
        {
            let (lock, _cvar) = &*milk_foam_container;
            let milk_foam = lock.lock().unwrap();
            report.push_str(&format!(
                "Leche espumada actualmente: {} - Consumida: {} ",
                milk_foam.get_milk(),
                milk_foam.get_amount_used()
            ));
        }
        println!("{}", report);
        thread::sleep(Duration::from_secs(STATS_TIME));
    }
}

fn inform_about_milk_foam(milk_foam_container_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>) {
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
) {
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
) {
    loop {
        let (lock, cvar) = &*ground_coffee_beans_container_clone;
        let mut ground_coffee_beans = cvar
            .wait_while(lock.lock().unwrap(), |ground_coffee_beans| {
                ground_coffee_beans.has_any()
            })
            .unwrap();
        let coffee_beans_to_grind = coffee_beans_to_grind_container_clone.lock().unwrap();
        let value_to_refill = 100;
        convert_coffee_beans_to_ground_beans(
            &mut ground_coffee_beans,
            &value_to_refill,
            coffee_beans_to_grind,
        );
        cvar.notify_all();
    }
}

fn transform_milk(
    cold_milk_clone: Arc<Mutex<ColdMilkContainer>>,
    milk_foam_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
) {
    loop {
        let (lock, cvar) = &*milk_foam_clone;
        let mut milk_foam = cvar
            .wait_while(lock.lock().unwrap(), |milk_foam| milk_foam.has_any())
            .unwrap();
        let cold_milk = cold_milk_clone.lock().unwrap();
        let value_to_refill = 100;
        convert_milk_to_foam_milk(&mut milk_foam, &value_to_refill, cold_milk);
        cvar.notify_all();
    }
}

fn make_drink(
    n_dispenser: u64,
    ground_coffee_beans_clone: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
    milk_foam_clone: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
    total_drinks_prepared_clone: Arc<Mutex<i32>>,
    coffee_beans_to_grind_container_clone: Arc<Mutex<CoffeeBeansToGrindContainer>>,
    cold_milk_container_clone: Arc<Mutex<ColdMilkContainer>>,
) {
    loop {
        let coffee_order = random::<u64>() % 5;
        let milk_order = random::<u64>() % 5;
        let water_order = random::<u64>() % 5;
        println!("[Dispenser {}] Recibió orden de café con: {} gr de café, {} gr de leche y {} gr de agua", n_dispenser, coffee_order, milk_order, water_order);
        if coffee_order > 0 {
            let (lock, cvar) = &*ground_coffee_beans_clone;
            let mut ground_coffee_beans = lock.lock().unwrap();
            if !ground_coffee_beans.has_enough(&coffee_order) {
                println!(
                    "[Dispenser {}] No hay suficientes {} granos de café para preparar la bebida",
                    n_dispenser, coffee_order
                );
                let coffee_beans_to_grind_container =
                    coffee_beans_to_grind_container_clone.lock().unwrap();
                convert_coffee_beans_to_ground_beans(
                    &mut ground_coffee_beans,
                    &((coffee_order as f64 * 1.5) as u64),
                    coffee_beans_to_grind_container,
                );
            }
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
            // let mut milk_foam = cvar
            //     .wait_while(lock.lock().unwrap(), |milk_foam| {
            //         !milk_foam.has_enough(&milk_order)
            //     })
            //     .unwrap();
            let mut milk_foam = lock.lock().unwrap();
            if !milk_foam.has_enough(&milk_order) {
                println!(
                    "[Dispenser {}] No hay suficiente {} leche espumada para preparar la bebida",
                    n_dispenser, milk_order
                );
                let cold_milk_container = cold_milk_container_clone.lock().unwrap();
                convert_milk_to_foam_milk(
                    &mut milk_foam,
                    &((milk_order as f64 * 1.5) as u64),
                    cold_milk_container,
                );
            }
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
    ground_coffee_beans: &mut MutexGuard<GroundCoffeeBeansContainer>,
    value_to_refill: &u64,
    mut coffee_beans_to_grind: MutexGuard<CoffeeBeansToGrindContainer>,
) {
    println!(
        "[Refill de café] Convirtiendo {} de granos para moler a granos molidos",
        value_to_refill
    );
    thread::sleep(Duration::from_millis(
        BASE_TIME_RESOURCE_REFILL * value_to_refill,
    ));
    coffee_beans_to_grind.grind(value_to_refill);
    ground_coffee_beans.add(value_to_refill);
    println!("[Refill de café] Terminó de convertir granos de café");
}

fn convert_milk_to_foam_milk(
    milk_foam_container: &mut MutexGuard<MilkFoamContainer>,
    value_to_refill: &u64,
    mut cold_milk_container: MutexGuard<ColdMilkContainer>,
) {
    println!(
        "[Refill de leche espumada] Convirtiendo {} de leche a leche espumada",
        value_to_refill
    );
    thread::sleep(Duration::from_millis(
        BASE_TIME_RESOURCE_REFILL * value_to_refill,
    ));
    cold_milk_container.subtract(value_to_refill);
    milk_foam_container.add(value_to_refill);
    println!("[Refill de leche espumada] Terminó de convertir leche espumada");
}
