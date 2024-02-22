mod implementation;

pub struct Queue {
    items: Vec<Box<dyn Submittable>>,
}

pub trait Submittable: Send + Sync {
    fn is_last(&self) -> bool;
    fn run(&mut self) -> ();
    fn get_name(&self) -> String;
}
