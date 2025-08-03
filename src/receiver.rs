mod tcp_client;

use tcp_client::StdTcpClient;
use std::io::{self, Write};
use core_affinity;

fn main() {
    println!("=== TCP Receiver for SoftIRQ Testing ===");
    
    // Set CPU affinity to core 0
    let core_ids = core_affinity::get_core_ids().unwrap();
    if !core_ids.is_empty() {
        core_affinity::set_for_current(core_ids[0]);
        println!("Receiver set to CPU core 0");
    } else {
        println!("Warning: No CPU cores available");
    }
    
    // Get user input for cpu_yield setting
    print!("Enable CPU yield? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let cpu_yield = input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes";
    
    println!("CPU yield enabled: {}", cpu_yield);
    
    // Connect to the server
    let mut client = match StdTcpClient::new("127.0.0.1:8080", cpu_yield) {
        Ok(client) => {
            println!("Connected to server at 127.0.0.1:8080");
            client
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            return;
        }
    };
    
    // Receive loop - infinite busy loop
    let mut buffer = [0u8; 1024];
    
    println!("Starting infinite receive loop...");
    loop {
        match client.recv(&mut buffer) {
            Ok(Some(size)) => {
                let message = String::from_utf8_lossy(&buffer[..size]);
                println!("Received message: {}", message.trim());
            }
            Ok(None) => {
                // No data available, continue busy loop
                continue;
            }
            Err(e) => {
                println!("Error receiving data: {}", e);
                break;
            }
        }
    }
}