use crate::queue::Queue;

mod implementation;

pub struct RThreadPool {
    pool_capacity: usize,
    queue: Queue,
}

unsafe impl Sync for RThreadPool {}
