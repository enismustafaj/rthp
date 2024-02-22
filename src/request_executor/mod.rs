use std::{collections::HashMap, io::Write, net::TcpStream};

use log::warn;

use crate::{queue::Submittable, thp::RThreadPool};

use self::{implementation::RootExecutor, request_data::RequestData};

mod implementation;
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
            let exec = RootExecutor::new(stream);
            self.pool.submit(Box::new(exec));
        } else {
            warn!("No patter for request was found");
            stream.write_all("404".as_bytes()).unwrap();
        }
    }
}
