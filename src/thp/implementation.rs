use crate::queue::{Queue, Submittable};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};

use super::RThreadPool;

impl RThreadPool {
    pub fn new(pool_capacity: usize) -> Self {
        let qq = vec![];
        let queue = Queue::new(qq);

        let mut_pool: Arc<Mutex<Queue>> = Arc::new(Mutex::new(queue));
        let mut threads: Vec<JoinHandle<()>> = vec![];

        for k in 0..pool_capacity {
            let i = Arc::clone(&mut_pool);
            threads.push(thread::spawn(move || 'listen: {
                let name: String = String::from(format!("{}", k));
                println!("thread {} started", name);
                loop {
                    thread::sleep(time::Duration::from_millis(3000));
                    let mut j = i.lock().unwrap();

                    match j.de_q() {
                        Some(val) => {
                            if val.is_last() {
                                println!("worker {} thread stopped", name);
                                break 'listen;
                            }
                            println!("thread {} executed {}", name, val.get_name());
                            val.run();
                        }
                        None => {
                            println!("queue is empty");
                        }
                    }
                }
            }));
        }

        RThreadPool {
            pool_capacity,
            queue: mut_pool,
            workers: threads,
        }
    }

    pub fn submit(&mut self, element: Box<dyn Submittable>) -> () {
        let mut a = self.queue.lock().unwrap();

        match a.en_q(element) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

struct TaskStop {}

impl TaskStop {
    fn new() -> Self {
        Self {}
    }
}

impl Submittable for TaskStop {
    fn is_last(&self) -> bool {
        true
    }

    fn run(&self) -> () {}

    fn get_name(&self) -> &String {
        todo!()
    }
}

impl Drop for RThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.pool_capacity {
            self.submit(Box::new(TaskStop::new()));
            if let Some(thread) = self.workers.pop() {
                thread.join().unwrap();
            }
        }
    }
}
