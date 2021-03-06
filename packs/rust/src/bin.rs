use std::env;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, Error};
use std::thread;


fn reply(mut stream: TcpStream) -> Result<(), Error> {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello World, I'm a Rust app!</body></html>\r\n";
    stream.write(response)?;
    Ok(())
}

fn main() {
    let port = env::var("PORT").expect("$PORT not found");
    let listener = TcpListener::bind("127.0.0.1:".to_owned()+&port).unwrap();
    println!("👌 Listening for connections on port {}", port);
    for stream in listener.incoming() {

        match stream {

            Ok(stream) => {
                stream.set_nonblocking(true).expect("set_nonblocking call failed");
                stream.set_write_timeout(None).expect("set_write_timeout call failed");
                stream.set_nodelay(true).expect("set_nodelay call failed");
                thread::spawn(|| {
                    match reply(stream){
                        Ok(_) =>{},
                        Err(e) => println!("IO error: {}", e)
                    }
                });


            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
