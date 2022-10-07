use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_DISPENSERS: usize = 3;

fn main() {
    let coffee_beans_to_grind = Arc::new(Mutex::new(100));
    let ground_coffee_beans = Arc::new(Mutex::new(50));
    let milk_foam = Arc::new(Mutex::new(100));
    let cold_milk = Arc::new(Mutex::new(50));

    let dispensers: Vec<JoinHandle<()>> = (1..MAX_DISPENSERS + 1)
        .map(|i| {
            let ground_coffee_beans_clone = ground_coffee_beans.clone();
            let milk_foam_clone = milk_foam.clone();
            thread::spawn(move || loop {
                let coffee_order = 10;
                let milk_order = 20;
                let water_order = 30;

                make_coffee(
                    i,
                    &ground_coffee_beans_clone,
                    &milk_foam_clone,
                    coffee_order,
                    milk_order,
                    water_order,
                );
            })
        })
        .collect();

    let _: Vec<()> = dispensers
        .into_iter()
        .flat_map(|dispenser| dispenser.join())
        .collect();
}

fn make_coffee(
    n_dispenser: usize,
    ground_coffee_beans_clone: &Arc<Mutex<u64>>,
    milk_foam_clone: &Arc<Mutex<u64>>,
    coffee_order: u64,
    milk_order: u64,
    water_order: u64,
) {
    {
        let mut ground_coffee_beans = ground_coffee_beans_clone.lock().unwrap();
        println!(
            "Dispenser {}: Aplicando granos de café: {}",
            n_dispenser, coffee_order
        );
        thread::sleep(Duration::from_millis(100 * coffee_order));
        *ground_coffee_beans -= coffee_order;
        println!(
            "Dispenser {}: Terminó de aplicar granos de café",
            n_dispenser
        );
    }

    {
        let mut milk_foam = milk_foam_clone.lock().unwrap();
        println!(
            "Dispenser {}: Aplicando leche espumada: {}",
            n_dispenser, milk_order
        );
        thread::sleep(Duration::from_millis(100 * milk_order));
        println!(
            "Dispenser {}: Terminó de aplicar leche espumada",
            n_dispenser
        );
        *milk_foam -= milk_order;
    }

    {
        println!("Dispenser {}: Aplicando agua", n_dispenser);
        thread::sleep(Duration::from_millis(100 * water_order));
        println!("Dispenser {}: Terminó de aplicar agua", n_dispenser);
    }
    println!("Dispenser {}: Terminó de hacer café", n_dispenser);
}
