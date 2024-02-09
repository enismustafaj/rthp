use super::Queue;

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        Queue {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn en_q(&mut self, item: T) -> Result<(), ()> {
        if self.is_full() {
            return Err(());
        }
        self.items.push(item);

        Ok(())
    }

    pub fn de_q(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.items.remove(0);

        Some(item)
    }

    fn is_full(&self) -> bool {
        self.capacity == self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
