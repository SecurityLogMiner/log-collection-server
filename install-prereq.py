#This script serves as a method of installing all the required packages on the instance.
#Just GCC for now, may develop as time goes on.

import os
import platform
import subprocess


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