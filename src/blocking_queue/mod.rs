use std::collections::*;
use std::sync::*;

#[derive(Debug)]
pub struct BlockingQueue<T> {
    dequeue: Mutex<VecDeque<T>>,
    cvar: Condvar,
}

impl<T> BlockingQueue<T> {
    /// Create empty blocking queue
    pub fn new() -> Self {
        Self {
            dequeue: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }

    /// Push input on back of queue
    pub fn push_back(&self, value: T) {
        let mut dequeue = self.dequeue.lock().expect("Failed to lock dequeue");
        dequeue.push_back(value);
        self.cvar.notify_all();
    }

    /// Pop element from front of queue
    pub fn pop_front(&self) -> T {
        let mut dequeue = self
            .cvar
            .wait_while(
                self.dequeue.lock().expect("Failed to lock dequeue"),
                |dequeue| dequeue.is_empty(),
            )
            .expect("Failed to wait while dequeue is empty");
        dequeue.pop_front().expect("Failed to pop front of dequeue")
    }

}
