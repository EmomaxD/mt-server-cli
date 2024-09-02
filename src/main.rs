mod args;
mod server;
mod signal;
mod handler;

use args::parse_args;
use server::{start_tcp_server, start_udp_server};
use signal::setup_ctrlc_handler;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use threadpool::ThreadPool;
use num_cpus;

fn main() {
    let (server_type, port, save_path, shutdown_after) = parse_args();

    let num_cores = num_cpus::get();
    let pool = ThreadPool::new(2 * num_cores);
    let request_counter = Arc::new(Mutex::new(0));
    let bytes_received = Arc::new(Mutex::new(0));
    let bytes_sent = Arc::new(Mutex::new(0));
    let start_time = SystemTime::now();
    let address = format!("127.0.0.1:{}", port);

    setup_ctrlc_handler(save_path.clone(), request_counter.clone(), bytes_received.clone(), bytes_sent.clone(), start_time);

    match server_type.as_str() {
        "tcp" => start_tcp_server(&address, pool, request_counter, bytes_received, bytes_sent, shutdown_after),
        "udp" => start_udp_server(&address, pool, request_counter, bytes_received, bytes_sent, shutdown_after),
        _ => eprintln!("Unsupported server type"),
    }
}
