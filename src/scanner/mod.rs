use anyhow::{Result, Error};
use futures::Future;
use futures::{stream::FuturesUnordered, StreamExt};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Debug)]
pub struct Scanner {
    ip: IpAddr,
    batch_size: u16,
    timeout: Duration,
    tries: u8,
}

impl Scanner {
    pub fn new(ip: IpAddr) -> Self {
        Self {
            ip,
            batch_size: 4500,
            timeout: Duration::from_millis(1000),
            tries: 1,
        }
    }

    // Main scanning function
    pub async fn run(&self) -> Result<Vec<u16>> {
        let mut ports: Vec<u16> = (LOWEST_PORT_NUMBER..=TOP_PORT_NUMBER).collect();
        ports.shuffle(&mut thread_rng());

        let mut open_ports = Vec::new();
        let mut futures = FuturesUnordered::new();
        let mut errors = HashSet::with_capacity(TOP_PORT_NUMBER as usize);

        for &port in &ports {
            let socket = SocketAddr::new(self.ip, port);
            futures.push(self.scan_socket(socket));

            if futures.len() == self.batch_size as usize {
                self.process_futures(&mut futures, &mut open_ports, &mut errors)
                    .await?;
            }
        }

        // Process any remaining futures
        self.process_futures(&mut futures, &mut open_ports, &mut errors)
            .await?;

        Ok(open_ports)
    }

    // Process a batch of futures
    async fn process_futures(
        &self,
        futures: &mut FuturesUnordered<impl Future<Output = Result<SocketAddr, Error>>>,
        open_ports: &mut Vec<u16>,
        errors: &mut HashSet<String>,
    ) -> Result<()> {
        while let Some(result) = futures.next().await {
            match result {
                Ok(socket) => open_ports.push(socket.port()),
                Err(e) => {
                    errors.insert(e.to_string());
                }
            }
        }
        Ok(())
    }

    // Scan a single socket
    async fn scan_socket(&self, socket: SocketAddr) -> Result<SocketAddr> {
        for _ in 1..=self.tries {
            if let Ok(mut stream) = self.connect(socket).await {
                println!("Open {}", socket.to_string());
                if let Err(e) = stream.shutdown().await {
                    println!("Shutdown stream error {}", e);
                }
                return Ok(socket);
            }
        }
        Err(Error::msg(format!("Failed to connect to {}", socket.ip())))
    }

    // Attempt to connect to a socket with a timeout
    async fn connect(&self, socket: SocketAddr) -> io::Result<TcpStream> {
        timeout(self.timeout, TcpStream::connect(socket)).await?
    }
}
