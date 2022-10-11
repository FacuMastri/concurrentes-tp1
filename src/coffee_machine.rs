use crate::constants::{
    COFFEE_TO_REFILL, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_RESET,
    COLOR_YELLOW, MILK_TO_REFILL,
};
use crate::container::Container;
use crate::order_reader::OrderReader;
use crate::utils::converter::{refill_coffee, refill_milk};
use crate::utils::Message;
use crate::{
    BlockingQueue, Order, BASE_TIME_RESOURCE_APPLICATION, COFFEE_BEANS_ALERT_THRESHOLD,
    INITIAL_COFFEE_BEANS_TO_GRIND, INITIAL_COLD_MILK, INITIAL_GROUND_COFFEE_BEANS,
    INITIAL_MILK_FOAM, MAX_DISPENSERS, MILK_FOAM_ALERT_THRESHOLD, RESOURCE_ALERT_FACTOR,
    STATS_UPDATE_INTERVAL,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

/// Represents a coffee machine, with its corresponding containers and dispensers
pub struct CoffeeMachine {
    coffee_beans_to_grind_container: Arc<Mutex<Container>>,
    ground_coffee_beans_container: Arc<(Mutex<Container>, Condvar)>,
    cold_milk_container: Arc<Mutex<Container>>,
    milk_foam_container: Arc<(Mutex<Container>, Condvar)>,
    total_drinks_prepared: Arc<Mutex<u64>>,
    blocking_queue: Arc<BlockingQueue<Message>>,
    should_shutdown: Arc<AtomicBool>,
}

impl CoffeeMachine {
    /// Creates a new coffee machine
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            coffee_beans_to_grind_container: Arc::new(Mutex::new(Container::new(
                INITIAL_COFFEE_BEANS_TO_GRIND,
            ))),
            ground_coffee_beans_container: Arc::new((
                Mutex::new(Container::new(INITIAL_GROUND_COFFEE_BEANS)),
                Condvar::new(),
            )),
            cold_milk_container: Arc::new(Mutex::new(Container::new(INITIAL_COLD_MILK))),
            milk_foam_container: Arc::new((
                Mutex::new(Container::new(INITIAL_MILK_FOAM)),
                Condvar::new(),
            )),
            total_drinks_prepared: Arc::new(Mutex::new(0)),
            blocking_queue: Arc::new(BlockingQueue::new()),
            should_shutdown: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Public interface to start the coffee machine
    /// This method will start the following threads:
    /// - A thread to take the orders
    /// - A thread to prepare the orders
    /// - A thread to inform the stats
    /// - A thread to alert about coffee beans when under certain threshold
    /// - A thread to alert about milk foam when under certain threshold
    /// - A thread to handle the refill of coffee beans
    /// - A thread to handle the refill of milk foam
    pub fn start(self: &Arc<Self>) {
        let reader_handle = self.read_orders();
        let dispensers = self.prepare_orders();
        let milk_refill = self.refill_milk();
        let coffee_refill = self.refill_coffee();
        let alert_system_for_coffee = self.alert_for_coffee();
        let alert_system_for_milk = self.alert_for_milk();
        let inform_system = self.inform_system();

        let _: Vec<()> = dispensers
            .into_iter()
            .flat_map(|dispenser| dispenser.join())
            .collect();

        // Debo avisarle a los threads que deben finalizar una vez que todos los threads terminaron
        // sus pedidos.
        self.should_shutdown.store(true, Ordering::Relaxed);
        let (_lock, cvar) = &*self.ground_coffee_beans_container;
        cvar.notify_all();
        let (_lock, cvar) = &*self.milk_foam_container;
        cvar.notify_all();

        coffee_refill
            .join()
            .expect("Failed to join coffee_refill thread");
        milk_refill
            .join()
            .expect("Failed to join milk_refill thread");
        alert_system_for_coffee
            .join()
            .expect("Failed to join alert_system_for_coffee thread");
        alert_system_for_milk
            .join()
            .expect("Failed to join alert_system_for_milk thread");
        inform_system
            .join()
            .expect("Failed to join inform_system thread");
        reader_handle
            .join()
            .expect("Failed to join reader_handle thread");

        let report = self.obtain_stats();
        println!("{}", report);
    }

    /// Reads the orders from the standard input
    /// This method will start a thread that will read the orders from the standard input
    fn read_orders(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();

        thread::spawn(move || {
            coffee_machine_clone.read_orders_wrapper();
            println!(
                "{}[Lector de pedidos]{} - No hay más pedidos para leer",
                COLOR_BLUE, COLOR_RESET
            );
            coffee_machine_clone.send_shutdown_message();
        })
    }

    /// Sends a shutdown message to the blocking queue, in order to notify the dispensers
    /// that they should stop
    fn send_shutdown_message(self: &Arc<Self>) {
        // Para finalizar el programa y hacer un shutdown, debo comunicarle a los dispensers que ya no hay más pedidos.
        for _ in 0..MAX_DISPENSERS {
            self.blocking_queue.push_back(Message::Shutdown);
        }
    }

    /// Reads the orders from the standard input
    /// This method will read the orders from the standard input and send them to the blocking queue
    fn read_orders_wrapper(self: &Arc<Self>) {
        let order_reader = OrderReader::new(self.blocking_queue.clone());
        order_reader.read_from(io::stdin());
    }

    /// Prepares the orders
    /// This method will start a number of threads that will prepare the orders
    /// The number of threads is defined by the constant MAX_DISPENSERS
    fn prepare_orders(self: &Arc<Self>) -> Vec<JoinHandle<()>> {
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

    /// Prepares a drink
    /// This method will prepare a drink, using the resources from the containers
    /// If there is not enough resources, the thread will refill the containers accordingly
    fn make_drink(self: &Arc<Self>, n_dispenser: u64) {
        loop {
            let order = self.blocking_queue.pop_front();
            match order {
                Message::Job(order) => {
                    println!(
                        "{}[Dispenser {}]{} - Recibió pedido: {}",
                        COLOR_GREEN, n_dispenser, COLOR_RESET, order
                    );
                    self.prepare_drink(order, n_dispenser);
                    println!(
                        "{}[Dispenser {}]{} - Terminó de preparar bebida ✓",
                        COLOR_GREEN, n_dispenser, COLOR_RESET
                    );
                }
                Message::Shutdown => {
                    println!(
                        "{}[Dispenser {}]{} - No hay pedidos, apagando dispenser",
                        COLOR_GREEN, n_dispenser, COLOR_RESET
                    );
                    break;
                }
            }
        }
    }

    /// Prepares a drink
    /// This method will prepare a drink, using the resources from the containers
    fn prepare_drink(&self, order: Order, n_dispenser: u64) {
        let coffee_amount = order.get_coffee();
        let milk_amount = order.get_milk();
        let water_amount = order.get_water();

        if order.requires_coffee() {
            self.serve_coffee(coffee_amount, n_dispenser);
        }

        if order.requires_milk() {
            self.serve_milk(milk_amount, n_dispenser);
        }

        if order.requires_water() {
            self.serve_water(water_amount, n_dispenser);
        }

        self.increase_drinks_prepared();
    }

    /// Increases the number of drinks prepared
    fn increase_drinks_prepared(&self) {
        let mut total_drinks = self
            .total_drinks_prepared
            .lock()
            .expect("Failed to lock total_drinks");
        *total_drinks += 1;
    }

    /// Serves water to the drink
    fn serve_water(&self, water_amount: &u64, n_dispenser: u64) {
        println!(
            "{}[Dispenser {}]{} - Aplicando agua",
            COLOR_GREEN, n_dispenser, COLOR_RESET
        );
        thread::sleep(Duration::from_millis(
            (BASE_TIME_RESOURCE_APPLICATION * water_amount) as u64,
        ));
        println!(
            "{}[Dispenser {}]{} - Terminó de aplicar agua",
            COLOR_GREEN, n_dispenser, COLOR_RESET
        );
    }

    /// Serves milk to the drink
    /// This method will serve milk to the drink, if there is enough milk in the container
    /// If there is not enough milk, the method will refill the container
    fn serve_milk(&self, milk_amount: &u64, n_dispenser: u64) {
        let (lock, cvar) = &*self.milk_foam_container;
        let mut milk_foam = lock.lock().expect("Failed to lock milk_foam");
        if !milk_foam.has_enough(milk_amount) {
            println!(
                "{}[Dispenser {}]{} - No hay suficiente {} leche espumada para preparar la bebida",
                COLOR_GREEN, n_dispenser, COLOR_RESET, milk_amount
            );
            let cold_milk_container = self
                .cold_milk_container
                .lock()
                .expect("Failed to lock cold_milk_container");
            refill_milk(
                &mut milk_foam,
                &((*milk_amount as f64 * 1.5) as u64),
                cold_milk_container,
            );
        }
        println!(
            "{}[Dispenser {}]{} - Aplicando {} de leche espumada",
            COLOR_GREEN, n_dispenser, COLOR_RESET, milk_amount
        );
        thread::sleep(Duration::from_millis(
            (BASE_TIME_RESOURCE_APPLICATION * milk_amount) as u64,
        ));
        milk_foam.subtract(milk_amount);
        println!(
            "{}[Dispenser {}]{} - Terminó de aplicar leche espumada",
            COLOR_GREEN, n_dispenser, COLOR_RESET
        );
        cvar.notify_all();
    }

    /// Serves coffee to the drink
    /// This method will serve coffee to the drink, if there is enough coffee in the container
    /// If there is not enough coffee, the method will refill the container
    fn serve_coffee(&self, coffee_amount: &u64, n_dispenser: u64) {
        let (lock, cvar) = &*self.ground_coffee_beans_container;
        let mut ground_coffee_beans = lock.lock().expect("Failed to lock ground_coffee_beans");
        if !ground_coffee_beans.has_enough(coffee_amount) {
            println!(
                "{}[Dispenser {}]{} - No hay suficientes {} granos de café para preparar la bebida",
                COLOR_GREEN, n_dispenser, COLOR_RESET, coffee_amount
            );
            let coffee_beans_to_grind_container = self
                .coffee_beans_to_grind_container
                .lock()
                .expect("Failed to lock coffee_beans_to_grind_container");
            refill_coffee(
                &mut ground_coffee_beans,
                &((*coffee_amount as f64 * 1.5) as u64),
                coffee_beans_to_grind_container,
            );
        }
        println!(
            "{}[Dispenser {}]{} - Aplicando {} granos de café",
            COLOR_GREEN, n_dispenser, COLOR_RESET, coffee_amount
        );
        thread::sleep(Duration::from_millis(
            (BASE_TIME_RESOURCE_APPLICATION * coffee_amount) as u64,
        ));
        ground_coffee_beans.subtract(coffee_amount);
        println!(
            "{}[Dispenser {}]{} - Terminó de aplicar granos de café",
            COLOR_GREEN, n_dispenser, COLOR_RESET
        );
        cvar.notify_all();
    }

    /// Refills the milk container
    /// This method will refill the milk container, using the cold milk container
    fn transform_milk(&self) {
        loop {
            let (lock, cvar) = &*self.milk_foam_container;
            let mut milk_foam = cvar
                .wait_while(lock.lock().expect("Failed to obtain lock"), |milk_foam| {
                    milk_foam.has_any() && !self.should_shutdown.load(Ordering::Relaxed)
                })
                .expect("Failed to wait for milk_foam");
            if self.should_shutdown.load(Ordering::Relaxed) {
                break;
            }
            let cold_milk = self
                .cold_milk_container
                .lock()
                .expect("Failed to lock cold_milk_container");
            println!(
                "{}[Refill de leche espumada]{} - La leche espumada se ha agotado",
                COLOR_MAGENTA, COLOR_RESET,
            );
            refill_milk(&mut milk_foam, &MILK_TO_REFILL, cold_milk);
            cvar.notify_all();
        }
        println!(
            "{}[Refill de leche espumada]{} - Apagando refill de leche espumada",
            COLOR_MAGENTA, COLOR_RESET
        );
    }

    /// Refills the coffee container
    /// This method will refill the coffee container, using the coffee beans to grind container
    fn transform_coffee(&self) {
        loop {
            let (lock, cvar) = &*self.ground_coffee_beans_container;
            let mut ground_coffee_beans = cvar
                .wait_while(
                    lock.lock().expect("Failed to obtain lock"),
                    |ground_coffee_beans| {
                        ground_coffee_beans.has_any()
                            && !self.should_shutdown.load(Ordering::Relaxed)
                    },
                )
                .expect("Failed to wait for ground_coffee_beans");
            if self.should_shutdown.load(Ordering::Relaxed) {
                break;
            }
            println!(
                "{}[Refill de granos de café]{} - Los granos de café molido se han agotado",
                COLOR_CYAN, COLOR_RESET,
            );
            let coffee_beans_to_grind = self
                .coffee_beans_to_grind_container
                .lock()
                .expect("Failed to lock coffee_beans_to_grind_container");
            refill_coffee(
                &mut ground_coffee_beans,
                &COFFEE_TO_REFILL,
                coffee_beans_to_grind,
            );
            cvar.notify_all();
        }
        println!(
            "{}[Refill de café]{} - Apagando refill de granos de café",
            COLOR_CYAN, COLOR_RESET
        );
    }

    /// Spawns a thread that will refill the milk container
    fn refill_milk(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.transform_milk())
    }

    /// Spawns a thread that will refill the coffee container
    fn refill_coffee(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.transform_coffee())
    }

    /// Informs about the current status of the ground coffee beans container
    /// This method will print the current status of the ground coffee beans container
    /// It will print the current amount of coffee in the container when it is under the threshold
    fn inform_about_coffee_beans(&self) {
        loop {
            let (lock, cvar) = &*self.ground_coffee_beans_container;
            let ground_coffee_beans = cvar
                .wait_while(
                    lock.lock().expect("Failed to obtain lock"),
                    |ground_coffee_beans| {
                        ground_coffee_beans.has_enough(&(COFFEE_BEANS_ALERT_THRESHOLD as u64))
                            && !self.should_shutdown.load(Ordering::Relaxed)
                    },
                )
                .expect("Failed to wait for ground_coffee_beans");
            if self.should_shutdown.load(Ordering::Relaxed) {
                break;
            }
            println!(
                "{}[Alerta de recursos: café]{} - El nivel de granos de café es de {} (threshold de {}%)", COLOR_RED, COLOR_RESET,
                (*ground_coffee_beans).get_current_amount(),
                RESOURCE_ALERT_FACTOR * 100.0
            );
        }
        println!(
            "{}[Alerta de recursos: café]{} - Apagando alerta de recursos de café",
            COLOR_RED, COLOR_RESET
        );
    }
    /// Spawns a thread that will inform about the current status of the ground coffee beans container
    fn alert_for_coffee(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_about_coffee_beans())
    }
    /// Spawns a thread that will inform about the current status of the milk foam container
    fn alert_for_milk(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_about_milk_foam())
    }

    /// Informs about the current status of the milk foam container
    /// This method will print the current status of the milk foam container
    /// It will print the current amount of milk in the container when it is under the threshold
    fn inform_about_milk_foam(&self) {
        loop {
            let (lock, cvar) = &*self.milk_foam_container;
            let milk_foam = cvar
                .wait_while(lock.lock().expect("Failed to obtain lock"), |milk_foam| {
                    milk_foam.has_enough(&(MILK_FOAM_ALERT_THRESHOLD as u64))
                        && !self.should_shutdown.load(Ordering::Relaxed)
                })
                .expect("Failed to wait for milk_foam");
            if self.should_shutdown.load(Ordering::Relaxed) {
                break;
            }
            println!(
                "{}[Alerta de recursos: leche]{} El nivel de leche espumada es de {} (threshold de {}%)", COLOR_RED, COLOR_RESET,
                (*milk_foam).get_current_amount(),
                RESOURCE_ALERT_FACTOR * 100.0
            );
        }
        println!(
            "{}[Alerta de recursos: leche]{} - Apagando alerta de recursos de leche",
            COLOR_RED, COLOR_RESET
        );
    }

    /// Spawns a thread that will inform about the statistic of the coffee machine
    fn inform_system(self: &Arc<Self>) -> JoinHandle<()> {
        let coffee_machine_clone = self.clone();
        thread::spawn(move || coffee_machine_clone.inform_stats())
    }

    /// Informs about the statistic of the coffee machine
    fn inform_stats(&self) {
        while !self.should_shutdown.load(Ordering::Relaxed) {
            let report = self.obtain_stats();
            println!("{}", report);
            thread::sleep(Duration::from_secs(STATS_UPDATE_INTERVAL));
        }
        println!(
            "{}[Estadísticas]{} - Apagando informe del sistema",
            COLOR_YELLOW, COLOR_RESET
        );
    }

    #[allow(clippy::format_push_string)]
    /// Obtains the statistic of the coffee machine
    /// This method will return a string with the statistic of the coffee machine
    /// It will return the amount of coffee, coffee beans, milk and milk foam that has been used
    fn obtain_stats(&self) -> String {
        let mut report = String::from("\x1b[33m[Estadísticas]\x1b[0m - ");
        {
            let total_drinks = self
                .total_drinks_prepared
                .lock()
                .expect("Failed to lock total_drinks");
            report.push_str(&format!(
                "Total de bebidas preparadas: {} || ",
                total_drinks
            ));
        }
        {
            let (lock, _cvar) = &*self.ground_coffee_beans_container;
            let ground_coffee_beans = lock.lock().expect("Failed to lock ground_coffee_beans");
            report.push_str(&format!(
                "Café molido actualmente: {} - Consumido: {} || ",
                ground_coffee_beans.get_current_amount(),
                ground_coffee_beans.get_amount_used()
            ));
        }
        {
            let coffee_beans_to_grind = self
                .coffee_beans_to_grind_container
                .lock()
                .expect("Failed to lock coffee_beans_to_grind");
            report.push_str(&format!(
                "Café en grano actualmente: {} - Consumido: {} || ",
                coffee_beans_to_grind.get_current_amount(),
                coffee_beans_to_grind.get_amount_used()
            ));
        }
        {
            let cold_milk = self
                .cold_milk_container
                .lock()
                .expect("Failed to lock cold_milk");
            report.push_str(&format!(
                "Leche fría actualmente: {} - Consumida: {} || ",
                cold_milk.get_current_amount(),
                cold_milk.get_amount_used()
            ));
        }
        {
            let (lock, _cvar) = &*self.milk_foam_container;
            let milk_foam = lock.lock().expect("Failed to lock milk_foam");
            report.push_str(&format!(
                "Leche espumada actualmente: {} - Consumida: {} ",
                milk_foam.get_current_amount(),
                milk_foam.get_amount_used()
            ));
        }
        report
    }
}
