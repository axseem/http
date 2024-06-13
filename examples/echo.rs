use http::{Method, Request, Response, Status};
use std::{
    collections::HashMap, io::{BufRead, BufReader, Write}, net::TcpListener, thread
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        thread::spawn(move || -> std::io::Result<()> {
            let mut stream = stream?;
            let mut reader = BufReader::new(&mut stream);
            let buf: Vec<u8> = reader.fill_buf()?.to_vec();

            let res = if let Ok(req) = Request::from(&buf) {
                match (req.method, req.target.as_ref()) {
                    (Method::Post, "/echo") => {
                        Response {
                            version: http::Version::OneOne,
                            status: Status::Ok,
                            headers: HashMap::from([
                                ("Content-Length".to_string(), req.body.len().to_string()),
                                ("Content-Type".to_string(), "text/plain".to_string()),
                            ]),
                            body: req.body,
                        }
                    },
                    _ => {
                        Response {
                            version: http::Version::OneOne,
                            status: Status::NotFound,
                            headers: HashMap::new(),
                            body: Vec::new(),
                        }
                    }
                }
            } else {
                Response {
                    version: http::Version::OneOne,
                    status: Status::BadRequest,
                    headers: HashMap::new(),
                    body: Vec::new(),
                }
            };

            stream.write(&res.to_bytes())?;
            stream.shutdown(std::net::Shutdown::Both)
        });
    }
    Ok(())
}
