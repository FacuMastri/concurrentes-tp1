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
        let mut dequeue = self.dequeue.lock().expect("Failed to lock dequeue");
        while dequeue.len() == 0 {
            dequeue = self.cvar.wait(dequeue).expect("Failed to wait on dequeue");
        }
        dequeue.pop_front().expect("Failed to pop front of dequeue")
    }

    /// Return number of elements in queue
    pub fn len(&self) -> usize {
        self.dequeue.lock().expect("Failed to lock dequeue").len()
    }
}
