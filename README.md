<div align="center">
    <img alt="http.rs" width="384" src="./logo.svg">
    <p><i>Simple low level HTTP library for Rust</i></p>
</div>

# Why?

The project was created to learn Rust and explore Hypertext Transfer Protocol.

# Showcase

A minimal server that responds with a [“200 OK” status](https://datatracker.ietf.org/doc/html/rfc9110#name-200-ok) to every request.

```rs
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
```

```sh
> curl -i localhost:8080   
HTTP/1.1 200 OK
```

There a bit more examples in the [examples](./examples) directory.