use std::sync::Mutex;

use super::{Queue, Submittable};

impl Queue {
    pub fn new(qq: Vec<Box<dyn Submittable>>) -> Self {
        Queue {
            items: Mutex::new(qq),
        }
    }

    pub fn en_q(&mut self, item: Box<dyn Submittable>) -> Result<(), ()> {
        let mut guard = self.items.lock().unwrap();
        guard.push(item);

        Ok(())
    }

    pub fn de_q(&mut self) -> Option<Box<dyn Submittable>> {
        let mut guard = self.items.lock().unwrap();

        if guard.is_empty() {
            return None;
        }
        let item = guard.remove(0);

        Some(item)
    }
}
