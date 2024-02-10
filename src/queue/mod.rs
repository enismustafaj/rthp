use std::sync::Mutex;

mod implementation;

pub struct Queue {
    items: Mutex<Vec<Box<dyn Submittable>>>,
}

pub trait Submittable: Send + Sync {
    fn is_last(&self) -> bool;
    fn run(&self) -> ();
}
