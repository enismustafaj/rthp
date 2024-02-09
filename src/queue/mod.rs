mod implementation;

pub struct Queue<T> {
    capacity: usize,
    items: Vec<T>,
}
