use std::{io::{Read, Write}, net::TcpListener};

pub struct RequestHandler;

#[warn(unused_must_use)]
impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler 
    }

    pub fn handle(&self) -> () {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buff: String = String::new();
                    match stream.read_to_string(&mut buff) {
                        Ok(_) => {
                            let mut headers = [httparse::EMPTY_HEADER; 64];
                            let mut req = httparse::Request::new(&mut headers);

                            match req.parse(buff.as_bytes()) {
                                Ok(_) => {
                                    let request_rows: Vec<&str> = buff.split("\n").collect();
                                    let req_info: Vec<&str> = request_rows[0].split(" ").collect();
                                    let request_method: &str = req_info[0];
                                    let request_path: &str = req_info[1];
                                },
                                Err(_) => {
                                    println!("Error parsing the request");

                                    stream.write("HTTP/1.1 400 BAD REQUEST\r\n\r\n".as_bytes()).unwrap();
                                },
                            }

                        },
                        Err(_) => {
                            panic!("Error")
                        },
                    }
                },

                Err(_) => {
                    panic!("Error")
                },
            }
        }
    }
}