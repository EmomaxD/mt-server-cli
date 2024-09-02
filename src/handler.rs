use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub fn handle_tcp_request(
    stream: &mut TcpStream,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>
) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            let mut total_bytes_received = bytes_received.lock().unwrap();
            *total_bytes_received += size;
            
            let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            stream.write_all(response).expect("Failed to write response");
            
            let mut total_bytes_sent = bytes_sent.lock().unwrap();
            *total_bytes_sent += response.len();
            
            let mut counter = request_counter.lock().unwrap();
            *counter += 1;
        },
        Err(e) => eprintln!("Failed to read from TCP stream: {}", e),
    }
}

pub fn handle_udp_request(
    socket: &mut UdpSocket,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>
) {
    let mut buffer = [0; 1024];
    
    match socket.recv_from(&mut buffer) {
        Ok((size, src)) => { // `src` is the source address
            let mut total_bytes_received = bytes_received.lock().unwrap();
            *total_bytes_received += size;
            
            let response = b"Hello, UDP!";
            socket.send_to(response, src).expect("Failed to send response");
            
            let mut total_bytes_sent = bytes_sent.lock().unwrap();
            *total_bytes_sent += response.len();
            
            let mut counter = request_counter.lock().unwrap();
            *counter += 1;
        },
        Err(e) => eprintln!("Failed to receive from UDP socket: {}", e),
    }
}




pub fn log_metrics(
    path: String,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>,
    start_time: SystemTime,
    end_time: SystemTime
) {
    let mut file = File::create(path).expect("Failed to create log file");

    let elapsed = end_time
        .duration_since(start_time)
        .expect("Time went backwards")
        .as_secs();

    let counter = *request_counter.lock().unwrap();
    let received = *bytes_received.lock().unwrap();
    let sent = *bytes_sent.lock().unwrap();

    writeln!(file, "Log Start Time: {}", format_time(start_time))
        .expect("Failed to write to file");
    writeln!(file, "Log End Time: {}", format_time(end_time))
        .expect("Failed to write to file");
    writeln!(file, "Elapsed Time: {} seconds", elapsed)
        .expect("Failed to write to file");
    writeln!(file, "Metrics as of {} seconds since start:", elapsed)
        .expect("Failed to write to file");
    writeln!(file, "  Total requests: {}", counter)
        .expect("Failed to write to file");
    writeln!(file, "  Total bytes received: {}", received)
        .expect("Failed to write to file");
    writeln!(file, "  Total bytes sent: {}", sent)
        .expect("Failed to write to file");
}

fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.format("%H:%M:%S").to_string()
}

