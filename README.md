# Log Collection Server 

A service that listens for incoming log data froma specific source. The received
data is stored in a database.

This service also communicates with an api, using JWT to authenticate requests
on behalf of the client or server service.

## Table of Contents

- [Getting Started](#getting-started)
- [DB Setup](#db-setup)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

### Creating Issues
TODO

## Getting Started
Install Rust on your local machine. Use the following link to get setup quickly:
[rust setup](https://www.rust-lang.org/tools/install)

Clone the client and server repositories to start testing:
- [Client](https://github.com/SecurityLogMiner/log-collection-client)
- [Server](https://github.com/SecurityLogMiner/log-collection-server)

The client will read the configuration file and begin processing and sending 
log data from the given PATH to the server.

When running the client for the first time on a linux system, a directory will 
be created at:
- /var/log/logminer/logs/

If you do not have a system service that you are able to read log data from, you
can create one with a combination of a shell script and cronjob:

script.sh:
```
#!/bin/bash
for ((i = 1; i <= 60; i++)); do
    echo "test $(date)" >> /var/log/logminer/logs/test.log
    sleep 1
done
echo "" > /var/log/logminer/logs/test.log
```

cronjob:
```
* * * * * <path_to_your_script>
0 * * * * echo "" > /var/log/logminer/logs/test.log
```

If you do have a known path of streaming data to read from, supply that path to
the config file and start the server and client service - in that order.

Server:
```
cd <server_repo_dir>
cargo install
cargo run
```
Client:
```
cd <client_repo_dir>
cargo install
cargo run
```

You should see streaming data. This documentation will evolve as the project
progresses.

## Database Setup Requirements (testing):
- Postgresql 14 (or greater). havent tested any other version
NOTES:
- first dev run is on victoria 21.2. This should be compatible with the latest
  versions of ubuntu. Consider making an alpine instance and only installing 
  what is needed to run the server, database, and required packages. Ubuntu
  server is good enough if thats all we get to.

### DB Setup:

create user (see man createuser):
```
sudo -u postgres createuser -d -e -l testadmin
```

create database:
```
sudo -u testadmin createdb testadmin
```

add user to system:
```
sudo adduser testadmin
```

log into the psql server:
```
sudo -u testadmin psql
```

test database connection:
```
\conninfo
```
create the test table for test events:
```
create table time_event (
    time timestamp with time zone, // or time timestampz
    data text
);
```

On server startup, check if a database has been created and populated with the 
following tables:
- TimeEvent => time:data             timestampz:text
- UserCert  => username:cert         text:text
- UserIP    => username:IPs          text:[..., cidr] 

Upon successful connection to a client socket, the server begins writing to 
the corresponding table for that event time.

One thing to keep in mind is that the client may be sending in multiple paths
of log data. To accomodate this, the tables could be prefixed with the log path,
indicating its service (eg: auth_time_event or app_time_event).

### Postgresql Resources: 
- https://www.postgresql.org/docs/14/datatype.html

## License
Apache 2.0

## Acknowledgments
Syn Ack Fin

## Contact
Discord, if you know, you know

[Back to top](#table-of-contents)


