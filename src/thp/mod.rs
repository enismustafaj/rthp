use std::sync::{Arc, Mutex};

use crate::queue::Queue;

mod implementation;

pub struct RThreadPool {
    pool_capacity: usize,
    queue: Arc<Mutex<Queue>>,
}
