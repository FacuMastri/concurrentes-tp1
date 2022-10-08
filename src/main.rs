mod constants;
mod containers;
mod order;

use crate::constants::{COFFEE_BEANS_ALERT_THRESHOLD, MILK_FOAM_ALERT_THRESHOLD};
use constants::{BASE_TIME_RESOURCE_APPLICATION, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK, INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, RESOURCE_ALERT_FACTOR};
use rand::prelude::*;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::containers::CoffeeBeansToGrindContainer;

fn main() {
    let coffee_beans_to_grind_container = Arc::new(Mutex::new(CoffeeBeansToGrindContainer::new(INITIAL_COFFEE_BEANS_TO_GRIND)));
    let cold_milk_container = Arc::new(Mutex::new(INITIAL_COLD_MILK));

    let ground_coffee_beans_container =
        Arc::new((Mutex::new(INITIAL_GROUND_COFFEE_BEANS), Condvar::new()));
    let milk_foam_container = Arc::new((Mutex::new(INITIAL_MILK_FOAM), Condvar::new()));

    let total_drinks_prepared = Arc::new(Mutex::new(0));

    let dispensers: Vec<JoinHandle<()>> = (1..MAX_DISPENSERS + 1)
        .map(|i| {
            let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
            let milk_foam_clone = milk_foam_container.clone();
            let total_drinks_prepared_clone = total_drinks_prepared.clone();
            thread::spawn(move || loop {
                let coffee_order = rand::random::<u64>() % 5;
                let milk_order = rand::random::<u64>() % 5;
                let water_order = rand::random::<u64>() % 5;
                println!("[Dispenser {}] Recibió orden de café con: {} gr de café, {} gr de leche y {} gr de agua", i, coffee_order, milk_order, water_order);
                if coffee_order > 0 {
                    let (lock, cvar) = &*ground_coffee_beans_clone;
                    let mut ground_coffee_beans = cvar.wait_while(lock.lock().unwrap(), |&mut ground_coffee_beans| {
                        coffee_order > ground_coffee_beans
                    }).unwrap();
                    println!(
                        "[Dispenser {}] Aplicando granos de café: {}",
                        i, coffee_order
                    );
                    thread::sleep(Duration::from_millis((BASE_TIME_RESOURCE_APPLICATION * coffee_order) as u64));
                    *ground_coffee_beans -= coffee_order;
                    println!(
                        "[Dispenser {}] Terminó de aplicar granos de café",
                        i
                    );
                    cvar.notify_all();
                }

                if milk_order > 0 {
                    let (lock, cvar) = &*milk_foam_clone;
                    let mut milk_foam = cvar.wait_while(lock.lock().unwrap(), |&mut milk_foam| {
                        milk_order > milk_foam
                    }).unwrap();
                    println!("[Dispenser {}] Aplicando leche espumada: {}", i, milk_order);
                    thread::sleep(Duration::from_millis((BASE_TIME_RESOURCE_APPLICATION * milk_order) as u64));
                    *milk_foam -= milk_order;
                    println!("[Dispenser {}] Terminó de aplicar leche", i);
                    cvar.notify_all();
                }

                if water_order > 0 {
                    println!("[Dispenser {}] Aplicando agua", i);
                    thread::sleep(Duration::from_millis((BASE_TIME_RESOURCE_APPLICATION * water_order) as u64));
                    println!("[Dispenser {}] Terminó de aplicar agua", i);
                }

                println!("[Dispenser {}] Terminó de hacer café", i);
                let mut total_drinks = total_drinks_prepared_clone.lock().unwrap();
                *total_drinks += 1;
            })
        })
        .collect();

    let cold_milk_clone = cold_milk_container.clone();
    let milk_foam_clone = milk_foam_container.clone();

    let milk_refill = thread::spawn(move || loop {
        let (lock, cvar) = &*milk_foam_clone;
        let mut milk_foam = cvar
            .wait_while(lock.lock().unwrap(), |&mut milk_foam| milk_foam > 0)
            .unwrap();
        let value_to_refill = 100;
        println!(
            "[Refill de leche espumada] Convirtiendo {} de leche a leche espumada",
            value_to_refill
        );
        thread::sleep(Duration::from_millis(BASE_TIME_RESOURCE_APPLICATION * value_to_refill));
        let mut cold_milk = cold_milk_clone.lock().unwrap();
        *cold_milk -= value_to_refill;
        *milk_foam += value_to_refill;
        println!("[Refill de leche espumada] Terminó de convertir leche espumada");
        cvar.notify_all();
    });

    let ground_coffee_beans_container_clone = ground_coffee_beans_container.clone();
    let coffee_refill = thread::spawn(move || loop {
        let (lock, cvar) = &*ground_coffee_beans_container_clone;
        let mut ground_coffee_beans = cvar
            .wait_while(lock.lock().unwrap(), |&mut ground_coffee_beans| {
                ground_coffee_beans > 0
            })
            .unwrap();
        let value_to_refill = 100;
        println!(
            "[Refill de café] Convirtiendo {} de granos para moler a granos molidos",
            value_to_refill
        );
        thread::sleep(Duration::from_millis(1000 * value_to_refill));
        let mut coffee_beans_to_grind = coffee_beans_to_grind_container.lock().unwrap();
        coffee_beans_to_grind.grind(&value_to_refill);
        *ground_coffee_beans += value_to_refill;
        println!("[Refill de café] Terminó de convertir granos de café");
        cvar.notify_all();
    });

    let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
    let alert_system_for_coffee_beans = thread::spawn(move || loop {
        let (lock, cvar) = &*ground_coffee_beans_clone;
        let ground_coffee_beans = cvar
            .wait_while(lock.lock().unwrap(), |&mut ground_coffee_beans| {
                ground_coffee_beans > COFFEE_BEANS_ALERT_THRESHOLD as u64
            })
            .unwrap();
        println!(
                "[Alerta de recursos: café] El nivel de granos de café es de {} ({}% del total inicial, threshold de {}%)",
                *ground_coffee_beans, ((*ground_coffee_beans / INITIAL_GROUND_COFFEE_BEANS) as f64 * 100.0) as f64, RESOURCE_ALERT_FACTOR * 100.0
            );
    });

    let milk_foam_container_clone = milk_foam_container.clone();
    let alert_system_for_milk = thread::spawn(move || loop {
        let (lock, cvar) = &*milk_foam_container_clone;
        let milk_foam = cvar
            .wait_while(lock.lock().unwrap(), |&mut milk_foam| {
                milk_foam > MILK_FOAM_ALERT_THRESHOLD as u64
            })
            .unwrap();
        println!(
            "[Alerta de recursos: leche] El nivel de leche espumada es de {} ({}% del total inicial, threshold de {}%)",
            *milk_foam, ((*milk_foam / INITIAL_MILK_FOAM) as f64 * 100.0) as f64, RESOURCE_ALERT_FACTOR * 100.0
            );
    });

    coffee_refill.join().unwrap();
    milk_refill.join().unwrap();
    alert_system_for_coffee_beans.join().unwrap();
    alert_system_for_milk.join().unwrap();

    let _: Vec<()> = dispensers
        .into_iter()
        .flat_map(|dispenser| dispenser.join())
        .collect();
}
