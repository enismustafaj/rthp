use crate::thp::RThreadPool;
use std::fs;
use std::{collections::HashMap, io::Write, net::TcpStream};

use self::controller::Controller;

mod controller;

#[allow(dead_code)]
pub struct RequestExecutor {
    register: HashMap<String, String>,
    pool: RThreadPool,
}

impl RequestExecutor {
    pub fn new(register: HashMap<String, String>) -> Self {
        RequestExecutor {
            register,
            pool: RThreadPool::new(3),
        }
    }

    pub fn execute_request(&mut self, path: String, mut stream: TcpStream) {

        if !self.register.contains_key(&path) {
            self.return404(&mut stream);
        }
        let exec = Controller::new(stream, self.register.get(&path).unwrap().to_owned());
        self.pool.submit(Box::new(exec));
        
    }

    fn return404(&self, stream: &mut TcpStream) -> () {
            match fs::read_to_string("./src/static/404.html") {
                Ok(page) => {
                    let mut init: String = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                    init.push_str(page.as_str());
                    stream.write_all(init.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(_) => todo!(),
            }
    }
}
