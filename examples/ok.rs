use http::{Response, Status};
use std::{io::Write, net::{TcpListener, Shutdown}, thread};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        thread::spawn(move || -> std::io::Result<()> {
            let mut stream = stream?;
            let res = Response::from_status(Status::Ok);
            stream.write(&res.to_bytes())?;
            stream.shutdown(Shutdown::Both)
        });
    }
    Ok(())
}