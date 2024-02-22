use queue::Submittable;
use request_handler::RequestHandler;

mod queue;
mod request_executor;
mod request_handler;
mod thp;

struct Task {
    name: String,
}

#[allow(dead_code)]
impl Task {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Submittable for Task {
    fn is_last(&self) -> bool {
        false
    }

    fn run(&mut self) -> () {}

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    env_logger::init();

    let mut handler: RequestHandler = RequestHandler::new();

    handler.handle();
}
