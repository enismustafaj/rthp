use queue::Submittable;
use request_handler::RequestHandler;
use thp::RThreadPool;

mod queue;
mod thp;
mod request_handler;

struct Task {
    name: String,
}

impl Task {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Submittable for Task {
    fn is_last(&self) -> bool {
        false
    }

    fn run(&self) -> () {}

    fn get_name(&self) -> &String {
        &self.name
    }
}

fn main() {
    // let pool_capacity = 3;
    // let mut rthp = RThreadPool::new(pool_capacity);

    // for i in 0..10 {
    //     rthp.submit(Box::new(Task::new(String::from(format!("task {}", i)))));
    // }

    let handler: RequestHandler = RequestHandler::new();

    handler.handle();
}
