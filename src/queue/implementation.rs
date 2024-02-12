use super::{Queue, Submittable};

impl Queue {
    pub fn new(qq: Vec<Box<dyn Submittable>>) -> Self {
        Queue { items: qq }
    }

    pub fn en_q(&mut self, item: Box<dyn Submittable>) -> Result<(), ()> {
        self.items.push(item);

        Ok(())
    }

    pub fn de_q(&mut self) -> Option<Box<dyn Submittable>> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);

        Some(item)
    }
}
