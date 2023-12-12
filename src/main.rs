use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = &buffer[..size];
            println!("Rcvd data: {:?}", received_data);
            stream.write_all(received_data).unwrap();
        }
        Err(e) => {
            println!("Error reading from client: {}", e);
        }
    }
}

fn main() {
    let port = 44331;
    let listener = TcpListener::bind("127.0.0.1:44331")
                        .expect("failed to bind address");

    println!("log collection server running on 127.0.0.1:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("err accounting connection: {}", e);
            }
        }
    }
}
