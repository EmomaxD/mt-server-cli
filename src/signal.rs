use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use ctrlc;
use crate::handler::log_metrics;

pub fn setup_ctrlc_handler(
    save_path: Option<String>,
    request_counter: Arc<Mutex<usize>>,
    bytes_received: Arc<Mutex<usize>>,
    bytes_sent: Arc<Mutex<usize>>,
    start_time: SystemTime
) {
    ctrlc::set_handler(move || {
        if let Some(path) = save_path.clone() {
            let end_time = SystemTime::now();
            log_metrics(path, request_counter.clone(), bytes_received.clone(), bytes_sent.clone(), start_time, end_time);
        }
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
}
