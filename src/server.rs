use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Ok!");

                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            match Request::try_from(&buf[..]) {
                                Ok(request) => {}
                                Err(e) => print!("Failed to parse request: {}", e)
                            }

                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }

                }
                Err(e) => println!("Failed to establish connection: {}", e)
            }
        }
    }
}