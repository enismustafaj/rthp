use std::{io::Write, net::TcpStream};

use crate::queue::Submittable;

pub struct RootExecutor {
    stream: TcpStream,
}

impl RootExecutor {
    pub fn new(stream: TcpStream) -> Self {
        RootExecutor { stream }
    }
}

impl Submittable for RootExecutor {
    fn is_last(&self) -> bool {
        false
    }

    fn run(&mut self) -> () {
        self.stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();
    }

    fn get_name(&self) -> String {
        String::from("root handler")
    }
}
