# Installation

To get `keylogger` simply run the following in order:
```console
$ git clone https://github.com/NovusEdge/keylogger.git
$ cd keylogger/
```

# Usage

```
keylogger v0.1.0
Aliasgar Khimani (NovusEdge)
A simple keylogger written in rust

USAGE:
    keylogger [OPTIONS]

OPTIONS:
    -h, --help           Print help information
    -p, --path <path>    Specify the path of the file where the keystrokes will be logged.
    -V, --version        Print version information
```

```console
# To display help menu
$ cargo run -- -h 

# For custom path
$ cargo run -- --path <path>

# For logging into /tmp/keylog.log
$ cargo run
```
