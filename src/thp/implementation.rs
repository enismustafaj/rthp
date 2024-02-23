use super::RThreadPool;
use crate::queue::{Queue, Submittable};
use log::{debug, info};
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};

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
                info!("thread {} started", name);
                loop {
                    let mut random_generator = rand::thread_rng();
                    let waiting_time: u8 = random_generator.gen_range(1..255);
                    thread::sleep(time::Duration::from_millis(waiting_time as u64));

                    let mut j = i.lock().unwrap();

                    match j.de_q() {
                        Some(mut val) => {
                            if val.is_last() {
                                info!("worker {} thread stopped", name);
                                break 'listen;
                            }

                            debug!("{}", val.get_name());
                            val.run();
                        }
                        None => {}
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

    fn run(&mut self) -> () {}

    fn get_name(&self) -> String {
        todo!()
    }
}

impl Drop for RThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.pool_capacity {
            self.submit(Box::new(TaskStop::new()));
        }

        for _ in 0..self.pool_capacity {
            if let Some(thread) = self.workers.pop() {
                thread.join().unwrap();
            }
        }
    }
}
