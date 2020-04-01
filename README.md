# What is this?
This is a small file server for the HC800 computer. It presents a directory over a serial UART connection for fast build/test cycles of the firmware.

# How to use
    USAGE:
        uart-server [FLAGS] [OPTIONS] [PATH]

    FLAGS:
            --debug      Prints diagnostic messages
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -p, --port <PORT>    Serial port to use

    ARGS:
        <PATH>    The path to serve as filesystem (default current directory)


# Configuration
Specifying the root path and --port argument quickly gets tedious. To avoid having to do that, create a ```~/.uartfileserver``` (or ```C:\Users\{user}``` on Windows) file with the contents 

    port = "/dev/tty.usbserial-XXXXXXXX"
    path = "/Users/..."

Or on Windows:

    port = "COM3"
    path = "C:\\Users\\..."


# Installation
Currently you will need to have Rust (and Cargo) installed. Then run

    cargo install --force --path .

## Drivers
You may need to install a virtual COM port driver.

### macOS
https://www.ftdichip.com/Drivers/VCP.htm
