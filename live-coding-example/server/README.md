# Server example

This binary gets a command encoded json from udp and executes something

### Usage

```bash

$ ./server -h

target/debug/server --help
server 1.0
Erdem A. <erdemu.alici@gmail.com>
Gets a json formatted command and executes it

USAGE:
    server [OPTIONS] --port <port>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --n_worker <NumberOfWorkers>    How many worker threads (default 8)
    -p, --port <port>                   Sets the rx port
    
```