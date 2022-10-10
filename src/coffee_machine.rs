use crate::{
    BlockingQueue, CoffeeBeansToGrindContainer, ColdMilkContainer, GroundCoffeeBeansContainer,
    MilkFoamContainer, Order, BASE_TIME_RESOURCE_APPLICATION, BASE_TIME_RESOURCE_REFILL,
    COFFEE_BEANS_ALERT_THRESHOLD, INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK,
    INITIAL_GROUND_COFFEE_BEANS, INITIAL_MILK_FOAM, MAX_DISPENSERS, MILK_FOAM_ALERT_THRESHOLD,
    ORDER_TIME_ARRIVAL, RESOURCE_ALERT_FACTOR, STATS_UPDATE_TIME,
};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

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

#[derive(Clone, Copy, Debug)]
enum Ingredients {
    Coffee = 0,
    Milk,
    Water,
}

pub struct CoffeeMachine {
    coffee_beans_to_grind_container: Arc<Mutex<CoffeeBeansToGrindContainer>>,
    ground_coffee_beans_container: Arc<(Mutex<GroundCoffeeBeansContainer>, Condvar)>,
    cold_milk_container: Arc<Mutex<ColdMilkContainer>>,
    milk_foam_container: Arc<(Mutex<MilkFoamContainer>, Condvar)>,
    total_drinks_prepared: Arc<Mutex<u64>>,
    blocking_queue: Arc<BlockingQueue<Order>>,
}

impl CoffeeMachine {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            coffee_beans_to_grind_container: Arc::new(Mutex::new(
                CoffeeBeansToGrindContainer::new(INITIAL_COFFEE_BEANS_TO_GRIND),
            )),
            ground_coffee_beans_container: Arc::new((
                Mutex::new(GroundCoffeeBeansContainer::new(INITIAL_GROUND_COFFEE_BEANS)),
                Condvar::new(),
            )),
            cold_milk_container: Arc::new(Mutex::new(ColdMilkContainer::new(INITIAL_COLD_MILK))),
            milk_foam_container: Arc::new((
                Mutex::new(MilkFoamContainer::new(INITIAL_MILK_FOAM)),
                Condvar::new(),
            )),
            total_drinks_prepared: Arc::new(Mutex::new(0)),
            blocking_queue: Arc::new(BlockingQueue::new()),
        })
    }

    pub fn start(self: &Arc<Self>) {
        let reader_handle = self.read_orders();
        let dispensers = self.create_dispensers();
        let milk_refill = self.refill_milk();
        let coffee_refill = self.refill_coffee();
        let alert_system_for_coffee_beans = self.alert_for_coffee_beans();
        let alert_system_for_milk = self.alert_for_milk();
        let inform_system = self.inform_system();

        coffee_refill
            .join()
            .expect("Failed to join coffee_refill thread");
        milk_refill
            .join()
            .expect("Failed to join milk_refill thread");
        alert_system_for_coffee_beans
            .join()
            .expect("Failed to join alert_system_for_coffee_beans thread");
        alert_system_for_milk
            .join()
            .expect("Failed to join alert_system_for_milk thread");
        inform_system
            .join()
            .expect("Failed to join inform_system thread");
        reader_handle
            .join()
            .expect("Failed to join reader_handle thread");

        let _: Vec<()> = dispensers
            .into_iter()
            .flat_map(|dispenser| dispenser.join())
            .collect();
    }

    fn read_orders(&self) -> JoinHandle<()> {
        let blocking_queue_clone = self.blocking_queue.clone();

        thread::spawn(move || {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(io::stdin());
            for result in reader.records() {
                println!("[Lector de pedidos] Tomando pedido");
                let record = result.expect("Failed to read record");
                let order = Order::new(
                    record[Ingredients::Coffee as usize]
                        .parse()
                        .expect("Failed to parse coffee"),
                    record[Ingredients::Milk as usize]
                        .parse()
                        .expect("Failed to parse milk"),
                    record[Ingredients::Water as usize]
                        .parse()
                        .expect("Failed to parse water"),
                );
                println!("[Lector de pedidos] Pedido tomado y anotado: {:?}", order);
                blocking_queue_clone.push_back(order);
                // Sleep para simular que todos los pedidos no llegan de inmediato. Similar a clientes.
                thread::sleep(Duration::from_millis(ORDER_TIME_ARRIVAL));
            }
            // Para finalizar el programa y hacer un shutdown, debo comunicarle a los dispensadores que ya no hay más pedidos.
            println!("[Lector de pedidos] No hay más pedidos para leer");
            for _ in 0..MAX_DISPENSERS {
                blocking_queue_clone.push_back(Order::new(0, 0, 0));
            }
        })
    }

    fn create_dispensers(self: &Arc<Self>) -> Vec<JoinHandle<()>> {
        #[allow(clippy::needless_collect)]
        let dispensers: Vec<JoinHandle<()>> = (1..MAX_DISPENSERS + 1)
            .map(|i| {
                let coffee_machine_clone = self.clone();
                thread::spawn(move || {
                    coffee_machine_clone.make_drink(i);
                })
            })
            .collect();
        dispensers
    }

    fn make_drink(self: &Arc<Self>, n_dispenser: u64) {
        let ground_coffee_beans_container = self.ground_coffee_beans_container.clone();
        let milk_foam_container = self.milk_foam_container.clone();
        let total_drinks_prepared = self.total_drinks_prepared.clone();
        let coffee_beans_to_grind_container = self.coffee_beans_to_grind_container.clone();
        let cold_milk_container = self.cold_milk_container.clone();
        let blocking_queue = self.blocking_queue.clone();

        loop {
            let order = blocking_queue.pop_front();

            if order.is_empty() {
                println!(
                    "[Dispenser {}] No hay pedidos, apagando dispenser",
                    n_dispenser
                );
                break;
            }

            let coffee_amount = order.get_coffee();
            let milk_amount = order.get_milk();
            let water_amount = order.get_water();
            println!("[Dispenser {}] Recibió orden de: {:?}", n_dispenser, order);
            if order.requires_coffee() {
                let (lock, cvar) = &*ground_coffee_beans_container;
                let mut ground_coffee_beans =
                    lock.lock().expect("Failed to lock ground_coffee_beans");
                if !ground_coffee_beans.has_enough(coffee_amount) {
                    println!(
                        "[Dispenser {}] No hay suficientes {} granos de café para preparar la bebida",
                        n_dispenser, coffee_amount
                    );
                    let coffee_beans_to_grind_container = coffee_beans_to_grind_container
                        .lock()
                        .expect("Failed to lock coffee_beans_to_grind_container");
                    convert_coffee_beans_to_ground_beans(
                        &mut ground_coffee_beans,
                        &((*coffee_amount as f64 * 1.5) as u64),
                        coffee_beans_to_grind_container,
                    );
                }
                println!(
                    "[Dispenser {}] Aplicando granos de café: {}",
                    n_dispenser, coffee_amount
                );
                thread::sleep(Duration::from_millis(
                    (BASE_TIME_RESOURCE_APPLICATION * coffee_amount) as u64,
                ));
                ground_coffee_beans.subtract(coffee_amount);
                println!(
                    "[Dispenser {}] Terminó de aplicar granos de café",
                    n_dispenser
                );
                cvar.notify_all();
            }

            if order.requires_milk() {
                let (lock, cvar) = &*milk_foam_container;
                let mut milk_foam = lock.lock().expect("Failed to lock milk_foam");
                if !milk_foam.has_enough(milk_amount) {
                    println!(
                        "[Dispenser {}] No hay suficiente {} leche espumada para preparar la bebida",
                        n_dispenser, milk_amount
                    );
                    let cold_milk_container = cold_milk_container
                        .lock()
                        .expect("Failed to lock cold_milk_container");
                    convert_milk_to_foam_milk(
                        &mut milk_foam,
                        &((*milk_amount as f64 * 1.5) as u64),
                        cold_milk_container,
                    );
                }
                println!(
                    "[Dispenser {}] Aplicando leche espumada: {}",
                    n_dispenser, milk_amount
                );
                thread::sleep(Duration::from_millis(
                    (BASE_TIME_RESOURCE_APPLICATION * milk_amount) as u64,
                ));
                milk_foam.subtract(milk_amount);
                println!("[Dispenser {}] Terminó de aplicar leche", n_dispenser);
                cvar.notify_all();
            }

            if order.requires_water() {
                println!("[Dispenser {}] Aplicando agua", n_dispenser);
                thread::sleep(Duration::from_millis(
                    (BASE_TIME_RESOURCE_APPLICATION * water_amount) as u64,
                ));
                println!("[Dispenser {}] Terminó de aplicar agua", n_dispenser);
            }

            println!("[Dispenser {}] Terminó de hacer café", n_dispenser);
            let mut total_drinks = total_drinks_prepared
                .lock()
                .expect("Failed to lock total_drinks");
            *total_drinks += 1;
        }
    }

    fn transform_milk(&self) {
        let milk_foam_container = self.milk_foam_container.clone();
        let cold_milk_container = self.cold_milk_container.clone();
        loop {
            let (lock, cvar) = &*milk_foam_container;
            let mut milk_foam = cvar
                .wait_while(lock.lock().expect("Failed to obtain lock"), |milk_foam| {
                    milk_foam.has_any()
                })
                .expect("Failed to wait for milk_foam");
            let cold_milk = cold_milk_container
                .lock()
                .expect("Failed to lock cold_milk_container");
            let value_to_refill = 100;
            convert_milk_to_foam_milk(&mut milk_foam, &value_to_refill, cold_milk);
            cvar.notify_all();
        }
    }

    fn transform_coffee(&self) {
        let ground_coffee_beans_container = self.ground_coffee_beans_container.clone();
        let coffee_beans_to_grind_container = self.coffee_beans_to_grind_container.clone();
        loop {
            let (lock, cvar) = &*ground_coffee_beans_container;
            let mut ground_coffee_beans = cvar
                .wait_while(
                    lock.lock().expect("Failed to obtain lock"),
                    |ground_coffee_beans| ground_coffee_beans.has_any(),
                )
                .expect("Failed to wait for ground_coffee_beans");
            let coffee_beans_to_grind = coffee_beans_to_grind_container
                .lock()
                .expect("Failed to lock coffee_beans_to_grind_container");
            let value_to_refill = 100;
            convert_coffee_beans_to_ground_beans(
                &mut ground_coffee_beans,
                &value_to_refill,
                coffee_beans_to_grind,
            );
            cvar.notify_all();
        }
    }

    fn refill_milk(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.transform_milk())
    }
    fn refill_coffee(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.transform_coffee())
    }

    fn inform_about_coffee_beans(&self) {
        let ground_coffee_beans_container = self.ground_coffee_beans_container.clone();
        loop {
            let (lock, cvar) = &*ground_coffee_beans_container;
            let ground_coffee_beans = cvar
                .wait_while(
                    lock.lock().expect("Failed to obtain lock"),
                    |ground_coffee_beans| {
                        ground_coffee_beans.has_enough(&(COFFEE_BEANS_ALERT_THRESHOLD as u64))
                    },
                )
                .expect("Failed to wait for ground_coffee_beans");
            println!(
                "[Alerta de recursos: café] El nivel de granos de café es de {} (threshold de {}%)",
                (*ground_coffee_beans).get_coffee_beans(),
                RESOURCE_ALERT_FACTOR * 100.0
            );
        }
    }
    fn alert_for_coffee_beans(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_about_coffee_beans())
    }
    fn alert_for_milk(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_about_milk_foam())
    }

    fn inform_about_milk_foam(&self) {
        let milk_foam_container = self.milk_foam_container.clone();
        loop {
            let (lock, cvar) = &*milk_foam_container;
            let milk_foam = cvar
                .wait_while(lock.lock().expect("Failed to obtain lock"), |milk_foam| {
                    milk_foam.has_enough(&(MILK_FOAM_ALERT_THRESHOLD as u64))
                })
                .expect("Failed to wait for milk_foam");
            println!(
                "[Alerta de recursos: leche] El nivel de leche espumada es de {} (threshold de {}%)",
                (*milk_foam).get_milk(),
                RESOURCE_ALERT_FACTOR * 100.0
            );
        }
    }

    fn inform_system(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_stats())
    }

    #[allow(clippy::format_push_string)]
    fn inform_stats(&self) {
        let total_drinks_prepared = self.total_drinks_prepared.clone();
        let ground_coffee_beans_container = self.ground_coffee_beans_container.clone();
        let coffee_beans_to_grind_container = self.coffee_beans_to_grind_container.clone();
        let cold_milk_container = self.cold_milk_container.clone();
        let milk_foam_container = self.milk_foam_container.clone();

        loop {
            let mut report = String::from("[Estadísticas] ");
            {
                let total_drinks = total_drinks_prepared
                    .lock()
                    .expect("Failed to lock total_drinks");
                report.push_str(&format!(
                    "Total de bebidas preparadas: {} || ",
                    total_drinks
                ));
            }
            {
                let (lock, _cvar) = &*ground_coffee_beans_container;
                let ground_coffee_beans = lock.lock().expect("Failed to lock ground_coffee_beans");
                report.push_str(&format!(
                    "Café molido actualmente: {} - Consumido: {} || ",
                    ground_coffee_beans.get_coffee_beans(),
                    ground_coffee_beans.get_amount_used()
                ));
            }
            {
                let coffee_beans_to_grind = coffee_beans_to_grind_container
                    .lock()
                    .expect("Failed to lock coffee_beans_to_grind");
                report.push_str(&format!(
                    "Café en grano actualmente: {} - Consumido: {} || ",
                    coffee_beans_to_grind.get_beans(),
                    coffee_beans_to_grind.get_amount_used()
                ));
            }
            {
                let cold_milk = cold_milk_container
                    .lock()
                    .expect("Failed to lock cold_milk");
                report.push_str(&format!(
                    "Leche fría actualmente: {} - Consumida: {} || ",
                    cold_milk.get_milk(),
                    cold_milk.get_amount_used()
                ));
            }
            {
                let (lock, _cvar) = &*milk_foam_container;
                let milk_foam = lock.lock().expect("Failed to lock milk_foam");
                report.push_str(&format!(
                    "Leche espumada actualmente: {} - Consumida: {} ",
                    milk_foam.get_milk(),
                    milk_foam.get_amount_used()
                ));
            }
            println!("{}", report);
            thread::sleep(Duration::from_secs(STATS_UPDATE_TIME));
        }
    }
}
