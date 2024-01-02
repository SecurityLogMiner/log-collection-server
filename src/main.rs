mod database;
use std::net::{TcpListener, TcpStream};
use postgres::{Client, NoTls};
use postgres_types::Timestamp;
use chrono::{DateTime, Utc};
use std::io::{Read, Write};
use std::str;

// see readme

// this function needs to have a db connection established so the stream can write
// to it. 
fn handle_client(mut stream: TcpStream) {
    let mut client = Client::connect(
        "host=localhost user=testadmin password=testadmin", 
        NoTls).unwrap();

    let mut buffer = [0;1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = &buffer[..size];
            //db.write_all(received_data).unwrap();
            let s = match str::from_utf8(received_data) {
                Ok(val) => val,
                Err(_) => "error reading incoming data",
            };
            println!("Rcvd data: {:?}", s);
            let time = Utc::now().to_string(); 
            client.execute(
                "insert into time_event (time,data) values ($1,$2)",
                &[&time,&s],
            ).unwrap();
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
