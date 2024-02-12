use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use crate::queue::Queue;

mod implementation;

pub struct RThreadPool {
    pool_capacity: usize,
    queue: Arc<Mutex<Queue>>,
    workers: Vec<JoinHandle<()>>,
}

unsafe impl Sync for RThreadPool {}
