use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use core_affinity;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut counter = 1u64;
    
    loop {
        let message = format!("{}", counter);
        
        match stream.write_all(message.as_bytes()) {
            Ok(_) => {
                stream.flush()?;
                println!("I sent {}, sleep 5 seconds", counter);
                counter += 1;
                thread::sleep(Duration::from_secs(5));
            }
            Err(e) => {
                println!("Failed to send message: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Error> {
    // Set CPU affinity to core 1
    let core_ids = core_affinity::get_core_ids().unwrap();
    if core_ids.len() > 1 {
        core_affinity::set_for_current(core_ids[1]);
        println!("Server set to CPU core 1");
    } else {
        println!("Warning: Only {} CPU core(s) available", core_ids.len());
    }
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {:?}", stream.peer_addr());
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        println!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
    
    Ok(())
}