use crate::request_executor::RequestExecutor;
use log::{error, info};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    executors: RequestExecutor
}

#[warn(unused_must_use)]
impl Server {
    pub fn new(routes: HashMap<String, String>) -> Self {
        Server {
            executors: RequestExecutor::new(routes),
        }
    }

    fn get_request(&self, stream: &mut TcpStream) -> String {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        http_request.join("\n")
    }

    pub fn handle_requests(&mut self) -> () {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        info!("Server listening to port 7878");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut headers = [httparse::EMPTY_HEADER; 64];
                    let mut req = httparse::Request::new(&mut headers);

                    let buff = self.get_request(&mut stream);
                    match req.parse(buff.as_bytes()) {
                        Ok(_) => {
                            let request_rows: Vec<&str> = buff.split("\n").collect();
                            let req_info: Vec<&str> = request_rows[0].split(" ").collect();

                            let request_path: &str = req_info[1];

                            self.executors.execute_request(
                                request_path.into(),
                                stream,
                            )
                        }
                        Err(_) => {
                            error!("Error parsing the request");

                            stream
                                .write("HTTP/1.1 400 BAD REQUEST\r\n\r\n".as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        }
                    }
                }

                Err(_) => {
                    error!("Error")
                }
            }
        }
    }
}
