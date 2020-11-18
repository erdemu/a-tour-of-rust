# Client example

This binary reads a json file and sends its' contents to the designated port on local host.

### Usage

```bash

$ ./client -h

client 1.0
Erdem A. <erdemu.alici@gmail.com>
Sends contents of a file over udp to a server

USAGE:
    client [OPTIONS] --port <port> --file <file_path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file_path>    A file you wish to send
    -i, --ip <ip_addr>        Target ip (127.0.0.1 if not given)
    -p, --port <port>         Sets the destination port
    
```