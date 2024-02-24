use self::{controllers::root_controller::RootController, request_data::RequestData};
use crate::{queue::Submittable, thp::RThreadPool};
use log::warn;
use std::fs;
use std::{collections::HashMap, io::Write, net::TcpStream};

mod controllers;
pub mod request_data;

#[allow(dead_code)]
pub struct RequestExecutor {
    register: HashMap<RequestData, Box<dyn Submittable>>,
    pool: RThreadPool,
}

impl RequestExecutor {
    pub fn new(register: HashMap<RequestData, Box<dyn Submittable>>) -> Self {
        RequestExecutor {
            register,
            pool: RThreadPool::new(3),
        }
    }

    pub fn execute_request(&mut self, data: RequestData, mut stream: TcpStream) {
        if data.get_method() == "GET" && data.get_path() == "/" {
            let exec = RootController::new(stream);
            self.pool.submit(Box::new(exec));
        } else {
            warn!("No patter for request was found");

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
}
