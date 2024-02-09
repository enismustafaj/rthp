mod implementation;

pub struct Queue {
    capacity: usize,
    items: Vec<u32>,
}

unsafe impl Send for Queue {}
