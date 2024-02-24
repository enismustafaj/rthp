use std::fs;
use std::{io::Write, net::TcpStream};

use log::error;

use crate::queue::Submittable;

pub struct RootController {
    stream: TcpStream,
}

impl RootController {
    pub fn new(stream: TcpStream) -> Self {
        RootController { stream }
    }
}

impl Submittable for RootController {
    fn is_last(&self) -> bool {
        false
    }

    fn run(&mut self) -> () {
        match fs::read_to_string("./src/static/index.html") {
            Ok(page) => {
                let mut init: String = String::from("HTTP/1.1 200 \r\n\r\n");
                init.push_str(page.as_str());
                self.stream.write_all(init.as_bytes()).unwrap();
                self.stream.flush().unwrap();
            }
            Err(err) => {
                self.stream
                    .write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())
                    .unwrap();
                self.stream.flush().unwrap();
                error!("Error reading the file {}", err)
            }
        }
    }

    fn get_name(&self) -> String {
        String::from("root handler")
    }
}
