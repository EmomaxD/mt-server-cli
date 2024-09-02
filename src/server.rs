use std::net::{TcpListener, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use crate::handler::{handle_tcp_request, handle_udp_request};

pub fn start_tcp_server(
    address: &str,
    pool: ThreadPool,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>,
    shutdown_after: Option<usize>
) {
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    println!("TCP Server listening on {}", address);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        let request_counter = request_counter.clone();
        let bytes_received = bytes_received.clone();
        let bytes_sent = bytes_sent.clone();

        pool.execute(move || {
            handle_tcp_request(&mut stream, request_counter.clone(), bytes_received, bytes_sent);
            if *request_counter.lock().unwrap() >= shutdown_after.unwrap_or(usize::MAX) {
                println!("Shutting down after {} requests", shutdown_after.unwrap_or(usize::MAX));
                std::process::exit(0);
            }
        });
    }
}

pub fn start_udp_server(
    address: &str,
    pool: ThreadPool,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>,
    shutdown_after: Option<usize>
) {
    let socket = UdpSocket::bind(address).expect("Failed to bind UDP socket");
    println!("UDP Server listening on {}", address);

    loop {
        let mut socket = socket.try_clone().ok().unwrap();
        let request_counter = request_counter.clone();
        let bytes_received = bytes_received.clone();
        let bytes_sent = bytes_sent.clone();

        pool.execute(move || {
            handle_udp_request(&mut socket, request_counter.clone(), bytes_received, bytes_sent);
            if *request_counter.lock().unwrap() >= shutdown_after.unwrap_or(usize::MAX) {
                println!("Shutting down after {} requests", shutdown_after.unwrap_or(usize::MAX));
                std::process::exit(0);
            }
        });

        thread::sleep(Duration::from_millis(100));
    }
}
