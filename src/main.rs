use env_logger::Env;
use server::RequestHandler;

mod queue;
mod request_executor;
mod server;
mod thp;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_target(false)
        .init();

    let mut handler: RequestHandler = RequestHandler::new();

    handler.handle();
}
