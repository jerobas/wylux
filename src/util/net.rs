use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("Port {0} is already in use")]
    PortInUse(u16),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Timeout waiting for port {0} to become available")]
    Timeout(u16),
}

/// Check if a port is available (not in use).
pub fn is_port_available(port: u16) -> Result<bool, NetError> {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => Ok(false),
        Err(e) => Err(NetError::Io(e)),
    }
}

/// Wait for a TCP port to become available (connectable).
pub fn wait_for_port(
    host: &str,
    port: u16,
    timeout: Duration,
) -> Result<(), NetError> {
    let start = Instant::now();
    let check_interval = Duration::from_millis(100);

    loop {
        match TcpStream::connect((host, port)) {
            Ok(_) => return Ok(()),
            Err(_) => {
                if start.elapsed() >= timeout {
                    return Err(NetError::Timeout(port));
                }
                std::thread::sleep(check_interval);
            }
        }
    }
}

/// Check if multiple ports are available.
pub fn check_ports_available(ports: &[u16]) -> Result<Vec<u16>, NetError> {
    let mut in_use = Vec::new();
    for &port in ports {
        if !is_port_available(port)? {
            in_use.push(port);
        }
    }
    if in_use.is_empty() {
        Ok(vec![])
    } else {
        Err(NetError::PortInUse(in_use[0]))
    }
}
