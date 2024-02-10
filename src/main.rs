use queue::Submittable;
use thp::RThreadPool;

mod queue;
mod thp;

struct Task {}

impl Task {
    fn new() -> Self {
        Self {}
    }
}

impl Submittable for Task {
    fn is_last(&self) -> bool {
        false
    }

    fn run(&self) -> () {
        println!("task1");
    }
}

struct TaskStop {}

impl TaskStop {
    fn new() -> Self {
        Self {}
    }
}

impl Submittable for TaskStop {
    fn is_last(&self) -> bool {
        true
    }

    fn run(&self) -> () {}
}

fn main() {
    let pool_capacity = 3;
    let mut rthp = Box::new(RThreadPool::new(3));
    rthp.submit(Box::new(Task::new()));
    rthp.submit(Box::new(Task::new()));

    for _ in 0..pool_capacity {
        rthp.submit(Box::new(TaskStop::new()))
    }

    RThreadPool::execute(rthp);
}
