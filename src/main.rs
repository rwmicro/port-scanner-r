use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::IpAddr;
use std::error::Error;
use clap::Parser;
use futures::future::join_all;

async fn scan_ports(ip: IpAddr, start_port: u16, end_port: u16) {
    let tasks: Vec<_> = (start_port..=end_port)
        .map(|port| {
            let ip = ip.clone();
            tokio::spawn(async move {
                match check_port(&ip.to_string(), port).await {
                    Ok(true) => println!("Port {} is open", port),
                    Ok(false) => (), // Port is closed
                    Err(e) => println!("Failed to scan port {}: {}", port, e),
                }
            })
        })
        .collect();

    join_all(tasks).await;
}

async fn check_port(address: &str, port: u16) -> Result<bool, Box<dyn Error>> {
    let target = format!("{}:{}", address, port);
    // Set a timeout to avoid long waits on closed ports
    let timeout_duration = Duration::from_secs(2);

    // Try connecting to the port
    match timeout(timeout_duration, TcpStream::connect(&target)).await {
        Ok(Ok(_)) => Ok(true),    // Port is open
        Ok(Err(e)) => Err(Box::new(e)),  // Error occurred
        Err(_) => Ok(false),      // Timeout (port is closed or unreachable)
    }
}

#[derive(Parser)]
struct Cli {
    /// The IP address to scan
    ip: String,

    /// Start port
    start_port: u16,

    /// End port
    end_port: u16,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let ip: IpAddr = args.ip.parse().expect("Invalid IP address");
    scan_ports(ip, args.start_port, args.end_port).await;
}

