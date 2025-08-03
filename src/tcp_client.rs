use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
#[derive(Debug)]
pub struct StdTcpClient {
    stream: TcpStream,
    cpu_yield: bool,
}

impl StdTcpClient {
    pub fn new<A: ToSocketAddrs>(
        address: A,
        cpu_yield: bool,
    ) -> Result<Self, Error> {
        let stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?; // Disable Nagle's algorithm for low latency
        stream.set_nonblocking(true)?;
        let res = Self {
            stream,
            cpu_yield,
        };
        Ok(res)
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, address: A) -> Result<(), Error> {
        self.stream = TcpStream::connect(address)?;
        self.stream.set_nodelay(true)?; // Disable Nagle's algorithm for low latency
        self.stream.set_nonblocking(true)?;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.stream.peer_addr().is_ok()
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let n = self.stream.write(buf)?;
        self.stream.flush()?;
        Ok(n)
    }

    /// non-blocking receive method
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Error> {
        match self.stream.read(buf) {
            Ok(0) => Err(Error::new(
                ErrorKind::ConnectionAborted,
                "Connection closed by the peer",
            )),
            Ok(size) => { 
                if self.cpu_yield {
                    std::thread::yield_now(); // Yield CPU if configured
                }
                Ok(Some(size)) 
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(None), // No data available
            Err(e) => Err(e), // Other errors
        }
    }
}

impl From<&str> for StdTcpClient {
    fn from(address: &str) -> Self {
        Self::new(address, true).expect("Failed to create TCP client")
    }
}


