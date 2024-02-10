use crate::queue::{Queue, Submittable};
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

use super::RThreadPool;

impl RThreadPool {
    pub fn new(pool_capacity: usize) -> Self {
        let qq = vec![];
        let queue = Queue::new(qq);

        RThreadPool {
            pool_capacity,
            queue,
        }
    }

    pub fn submit(&mut self, element: Box<dyn Submittable>) -> () {
        match self.queue.en_q(element) {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn execute(pool: Box<RThreadPool>) -> () {
        let len = pool.pool_capacity;
        let mut_pool = Arc::new(Mutex::new(pool.queue));
        for _ in 0..len {
            let i = Arc::clone(&mut_pool);
            thread::spawn(move || 'listen: loop {
                let mut j = i.lock().unwrap();

                match j.de_q() {
                    Some(val) => {
                        if val.is_last() {
                            println!("worker thread stopped");
                            break 'listen;
                        }
                        val.run();
                    }
                    None => {
                        thread::sleep(time::Duration::from_millis(100));
                    }
                }
            });
        }
    }
}
