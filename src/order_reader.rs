use crate::constants::{COLOR_BLUE, COLOR_RESET};
use crate::utils::{Message, Resource};
use crate::{BlockingQueue, Order, ORDER_TIME_INTERVAL_ARRIVAL};
use std::io::Read;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Reads orders from input_stream and pushes them to the output_queue.
pub struct OrderReader {
    output_queue: Arc<BlockingQueue<Message>>,
}

impl OrderReader {
    /// Create new order reader
    pub fn new(output_stream: Arc<BlockingQueue<Message>>) -> Self {
        Self {
            output_queue: output_stream,
        }
    }

    /// Starts reading orders from input_stream and pushing them to the output_queue.
    pub fn read_from(&self, input_stream: impl Read) {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(input_stream);
        for result in reader.records() {
            println!(
                "{}[Lector de pedidos]{} - Tomando pedido",
                COLOR_BLUE, COLOR_RESET
            );
            let record = result.expect("Failed to read record");
            let order = Order::new(
                record[Resource::Coffee as usize]
                    .parse()
                    .expect("Failed to parse coffee"),
                record[Resource::Milk as usize]
                    .parse()
                    .expect("Failed to parse milk"),
                record[Resource::Water as usize]
                    .parse()
                    .expect("Failed to parse water"),
            );
            println!(
                "{}[Lector de pedidos]{} - Pedido tomado y anotado: {}",
                COLOR_BLUE, COLOR_RESET, order
            );
            self.output_queue.push_back(Message::Job(order));
            // Sleep para simular que todos los pedidos no llegan de inmediato. Similar a clientes.
            thread::sleep(Duration::from_millis(ORDER_TIME_INTERVAL_ARRIVAL));
        }
    }
}
