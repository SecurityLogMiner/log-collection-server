# Log Collection Server 

A service that listens for incoming log data from a specific source. The received
data is stored in a database.

This service also communicates with an api, using JWT to authenticate requests
on behalf of the client or server service.

## Table of Contents

- [Getting Started](#getting-started)
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
To check that your instance has the required packages and to install them, you can 
run the install-prereq.py file.

install-prereq.py:
```
def install_gcc():
    if platform.dist()[0].lower() == 'ubuntu' and platform.dist()[1].startswith('22'):
        print('Ubuntu 22 detected. Installing gcc...')
        os.system('sudo apt update && sudo apt install gcc')
    else:
        print('This script only installs gcc on Ubuntu 22.')


def install_rust():
    print('Installing Rust...')
    os.system('curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh')

if __name__ == '__main__':
    print('Checking system and installing packages...')
    install_gcc()
    #Uncomment the next line to install Rust 
    #install_rust()
    print('Installation process complete.')
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

## License
Apache 2.0

## Acknowledgments
Syn Ack Fin

## Contact
Discord, if you know, you know

[Back to top](#table-of-contents)



