use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;


// this function needs to have a db connection established so the stream can write
// to it. 
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    //let mut db = std::fs::File::create("example.db").unwrap();
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = &buffer[..size];
            //db.write_all(received_data).unwrap();
            let s = match str::from_utf8(received_data) {
                Ok(val) => val,
                Err(_) => "error reading incoming data",
            };
            println!("Rcvd data: {:?}", s);
        }
        Err(e) => {
            println!("Error reading from client: {}", e);
        }
    }
}

fn main() {
    let port = 44331;
    let listener = TcpListener::bind("0.0.0.0:44331")
                        .expect("failed to bind address");

    println!("log collection server running on 0.0.0.0:{}", port);

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
