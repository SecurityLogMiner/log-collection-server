use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

/*
 * Requirements:
 *  - Postgresql 14 (or greater). havent tested any other version
 * NOTES:
 *  - first dev run is on victoria 21.2. This should be compatible with the latest
 *      versions of ubuntu. Consider making an alpine instance and only installing 
 *      what is needed to run the server, database, and required packages. Ubuntu
 *      server is good enough if thats all we get to.
 *
 *  DB Setup:
 *
 *      create user (see man createuser):
 *          sudo -u postgres createuser -d -e -l testadmin
 *
 *      create database:
 *          sudo -u testadmin createdb testadmin
 *
 *      add user to system:
 *          sudo adduser testadmin
 *
 *      log into the psql server:
 *          sudo -u testadmin psql
 *
 *      test database connection:
 *          \conninfo
 *      
 *      create the test table for test events:
 *          create table time_event (
 *              time timestamp with time zone, // or time timestampz
 *              data text
 *          );
 *      
 *      data types: https://www.postgresql.org/docs/14/datatype.html
 *
 * On server startup, check if a database has been created and populated with the 
 * following tables:
 *  - TimeEvent => time:data             timestampz:text
 *  - UserCert  => username:cert         text:text
 *  - UserIP    => username:IPs          text:[..., cidr] 
 * 
 * Upon successful connection to a client socket, the server begins writing to 
 * the corresponding table for that event time.
 *
 * One thing to keep in mind is that the client may be sending in multiple paths
 * of log data. To accomodate this, the tables could be prefixed with the log path,
 * indicating its service (eg: auth_time_event or app_time_event).
 */


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
