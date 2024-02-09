use super::Queue;

impl Queue {
    pub fn new(capacity: usize, qq: Vec<u32>) -> Self {
        Queue {
            capacity,
            items: qq,
        }
    }

    pub fn en_q(&mut self, item: u32) -> Result<(), ()> {
        if self.is_full() {
            return Err(());
        }
        self.items.push(item);

        Ok(())
    }

    pub fn de_q(&mut self) -> Option<u32> {
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
