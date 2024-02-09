use crate::queue::Queue;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use super::RThreadPool;

impl RThreadPool {
    pub fn new(pool_capacity: usize) -> Self {
        let qq = Vec::with_capacity(pool_capacity);
        let queue = Arc::new(Mutex::new(Queue::new(pool_capacity, qq)));

        RThreadPool {
            pool_capacity,
            queue,
        }
    }

    pub fn submit(&self, element: u32) -> () {
        let mut guard: MutexGuard<Queue> = self.queue.lock().unwrap();
        match guard.en_q(element) {
            Ok(_) => {
                println!("submitted to queue")
            }
            Err(_) => {
                println!("the queue is full")
            }
        }
    }

    pub fn execute<'a>(pool: &'a Box<RThreadPool>) -> () {
        for _ in 1..pool.pool_capacity {
            let qq = Arc::clone(&pool.queue);
            thread::spawn(move || loop {
                let mut que = qq.lock().unwrap();
                match que.de_q() {
                    Some(val) => {
                        println!("{}", val)
                    }
                    None => {}
                }
            });
        }
    }
}
