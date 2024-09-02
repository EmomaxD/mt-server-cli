# mt-server-cli

`mt-server-cli` is a command-line tool for creating a multi-threaded concurrent TCP or UDP server in Rust. It allows you to set up a server with request logging, automatic shutdown after a specified number of requests, and can be configured to save metrics to a file upon termination.

## Features

- Supports both TCP and UDP protocols.
- Configurable port, request logging, and automatic shutdown.
- Metrics logging.

## Usage

### Command Format

```sh
cargo run <tcp|udp> [--port <port>] [--save <file>] [--shutdown_after <num_requests>]
```

### Examples

```sh
cargo run tcp
```

```sh
cargo run udp --port 9090 --save metrics.log --shutdown_after 100
```

```sh
cargo run tcp --port 3030 --save server.log
```