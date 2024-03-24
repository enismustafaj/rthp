use std::collections::HashMap;

use env_logger::Env;
use server::Server;

mod queue;
mod request_executor;
mod server;
mod thp;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_target(false)
        .init();

    let mut routes: HashMap<String, String> = HashMap::new();
    compute_routes(&mut routes);

    let mut server: Server = Server::new(routes);

    server.handle_requests();
}

fn compute_routes(routes: &mut HashMap<String, String>) -> () {
    routes.insert("/".into(), "./src/static/index.html".into());
}


