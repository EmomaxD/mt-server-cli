use std::env;

pub fn parse_args() -> (String, u16, Option<String>, Option<usize>) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server_type> [--port <port>] [--save <file>] [--shutdown_after <num_requests>]", args[0]);
        std::process::exit(1);
    }

    let server_type = args[1].clone();
    let port = args.iter().position(|x| x == "--port")
        .map(|i| args[i + 1].parse::<u16>().unwrap_or(8080))
        .unwrap_or(8080);
    let save_path = args.iter().position(|x| x == "--save")
        .map(|i| args[i + 1].clone());
    let shutdown_after = args.iter().position(|x| x == "--shutdown_after")
        .map(|i| args[i + 1].parse().unwrap_or(usize::MAX));

    (server_type, port, save_path, shutdown_after)
}
