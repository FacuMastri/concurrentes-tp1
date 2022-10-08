use rand::prelude::*;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_DISPENSERS: u64 = 3;
const INITIAL_MILK_FOAM: u64 = 100;
const INITIAL_GROUND_COFFEE_BEANS: u64 = 100;
const RESOURCE_ALERT_THRESHOLD: f64 = 0.5;


fn main() {

    let coffee_beans_to_grind_container = Arc::new(Mutex::new(100000));
    let cold_milk_container = Arc::new(Mutex::new(100000));

    let ground_coffee_beans_container = Arc::new((Mutex::new(INITIAL_GROUND_COFFEE_BEANS), Condvar::new()));
    let milk_foam_container = Arc::new((Mutex::new(INITIAL_MILK_FOAM), Condvar::new()));


    let dispensers: Vec<JoinHandle<()>> = (1..MAX_DISPENSERS + 1)
        .map(|i| {
            let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
            let milk_foam_clone = milk_foam_container.clone();
            thread::spawn(move || loop {
                let coffee_order = rand::random::<u64>() % 5;
                let milk_order = rand::random::<u64>() % 5;
                let water_order = rand::random::<u64>() % 5;
                println!("[Dispenser {}]: Recibió orden de café con: {} gr de café, {} gr de leche y {} gr de agua", i, coffee_order, milk_order, water_order);
                if coffee_order > 0 {
                    let (lock, cvar) = &*ground_coffee_beans_clone;
                    let mut ground_coffee_beans = cvar.wait_while(lock.lock().unwrap(), |&mut ground_coffee_beans| {
                        coffee_order > ground_coffee_beans
                    }).unwrap();
                    println!(
                        "[Dispenser {}]: Aplicando granos de café: {}",
                        i, coffee_order
                    );
                    thread::sleep(Duration::from_millis((100 * coffee_order) as u64));
                    *ground_coffee_beans -= coffee_order;
                    println!(
                        "[Dispenser {}]: Terminó de aplicar granos de café",
                        i
                    );
                    cvar.notify_all();
                }

                if milk_order > 0 {
                    let (lock, cvar) = &*milk_foam_clone;
                    let mut milk_foam = cvar.wait_while(lock.lock().unwrap(), |&mut milk_foam| {
                        milk_order > milk_foam
                    }).unwrap();
                    println!("[Dispenser {}]: Aplicando leche espumada: {}", i, milk_order);
                    thread::sleep(Duration::from_millis((100 * milk_order) as u64));
                    *milk_foam -= milk_order;
                    println!("[Dispenser {}]: Terminó de aplicar leche", i);
                    cvar.notify_all();
                }

                if water_order > 0 {
                    println!("[Dispenser {}]: Aplicando agua", i);
                    thread::sleep(Duration::from_millis((100 * water_order) as u64));
                    println!("[Dispenser {}]: Terminó de aplicar agua", i);
                }

                println!("[Dispenser {}]: Terminó de hacer café", i);
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
            "[Refill de leche espumada]: Convirtiendo {} de leche a leche espumada",
            value_to_refill
        );
        thread::sleep(Duration::from_millis(100));
        let mut cold_milk = cold_milk_clone.lock().unwrap();
        *cold_milk -= value_to_refill;
        *milk_foam += value_to_refill;
        println!("[Refill de leche espumada]: Terminó de convertir leche espumada");
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
            "[Refill de café]: Convirtiendo {} de granos para moler a granos molidos",
            value_to_refill
        );
        thread::sleep(Duration::from_millis(100));
        let mut coffee_beans_to_grind = coffee_beans_to_grind_container.lock().unwrap();
        *coffee_beans_to_grind -= value_to_refill;
        *ground_coffee_beans += value_to_refill;
        println!("[Refill de café]: Terminó de convertir granos de café");
        cvar.notify_all();
    });



    let ground_coffee_beans_clone = ground_coffee_beans_container.clone();
    let alert_system_for_coffee_beans = thread::spawn(move || {
        loop {
            let (lock, cvar) = &*ground_coffee_beans_clone;
            let ground_coffee_beans = cvar
                .wait_while(lock.lock().unwrap(), |&mut ground_coffee_beans| {
                    ground_coffee_beans > (INITIAL_GROUND_COFFEE_BEANS as f64 * RESOURCE_ALERT_THRESHOLD) as u64
                })
                .unwrap();
            let
            println!(
                "[Alerta de recursos]: El nivel de granos de café es de {} ({}% del total inicial, threshold de {}%)",
                *ground_coffee_beans, (*ground_coffee_beans / INITIAL_GROUND_COFFEE_BEANS) * 100, RESOURCE_ALERT_THRESHOLD * 100.0
            );
        }
    });

    let milk_foam_container_clone = milk_foam_container.clone();
    let alert_system_for_milk = thread::spawn(move || {
        loop {
            let (lock, cvar) = &*milk_foam_container_clone;
            let milk_foam = cvar
                .wait_while(lock.lock().unwrap(), |&mut milk_foam| {
                    milk_foam > (INITIAL_MILK_FOAM as f64 * RESOURCE_ALERT_THRESHOLD) as u64
                })
                .unwrap();
            println!(
                "[Alerta de recursos]: El nivel de leche espumada es de {} ({}% del total inicial, threshold de {}%)",
                *milk_foam, (*milk_foam / INITIAL_MILK_FOAM) * 100, RESOURCE_ALERT_THRESHOLD * 100.0
            );
        }
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
