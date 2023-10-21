use super::PortStrategy;
use crate::port_strategy::SerialRange;
use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, Shutdown, SocketAddr},
    num::NonZeroU8,
    time::Duration,
};

pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Debug)]
pub struct Scanner {
    ip: IpAddr,
    batch_size: u16,
    timeout: Duration,
    tries: NonZeroU8,
    port_strategy: PortStrategy,
}

impl Scanner {
    pub fn new(ip: IpAddr) -> Self {
        Self {
            batch_size: 500,
            timeout: Duration::from_millis(5000),
            tries: NonZeroU8::new(std::cmp::max(1, 1)).unwrap(),
            port_strategy: PortStrategy::Serial(SerialRange {
                start: LOWEST_PORT_NUMBER,
                end: TOP_PORT_NUMBER,
            }),
            ip,
        }
    }

    pub async fn run(&self) -> HashMap<IpAddr, Vec<u16>> {
        let ports: Vec<u16> = self.port_strategy.order();
        let mut open_ports: Vec<u16> = Vec::new();
        let mut ftrs = FuturesUnordered::new();
        let mut errors: HashSet<String> = HashSet::with_capacity(1000);

        for &port in &ports[0..std::cmp::min(self.batch_size as usize, ports.len())] {
            let socket = SocketAddr::new(self.ip, port);
            ftrs.push(self.scan_socket(socket));
        }

        while let Some(result) = ftrs.next().await {
            match result {
                Ok(socket) => open_ports.push(socket.port()),
                Err(e) => {
                    let error_string = e.to_string();
                    if errors.len() < 1000 {
                        errors.insert(error_string);
                    }
                }
            }
        }

        let mut ports_per_ip: HashMap<IpAddr, Vec<u16>> = HashMap::new();
        ports_per_ip.insert(self.ip, open_ports);

        println!(
            "Open ports found for {}: {:?}",
            self.ip, ports_per_ip[&self.ip]
        );

        ports_per_ip
    }

    async fn scan_socket(&self, socket: SocketAddr) -> io::Result<SocketAddr> {
        let tries = self.tries.get();

        for nr_try in 1..=tries {
            match self.connect(socket).await {
                Ok(x) => {
                    if let Err(e) = x.shutdown(Shutdown::Both) {
                        println!("Shutdown stream error {}", &e);
                    }
                    println!("Open {}", socket.to_string());
                    return Ok(socket);
                }
                Err(e) => {
                    let mut error_string = e.to_string();

                    if nr_try == tries {
                        error_string.push(' ');
                        error_string.push_str(&socket.ip().to_string());
                        return Err(io::Error::new(io::ErrorKind::Other, error_string));
                    }
                }
            };
        }
        unreachable!();
    }

    async fn connect(&self, socket: SocketAddr) -> io::Result<TcpStream> {
        let stream = io::timeout(
            self.timeout,
            async move { TcpStream::connect(socket).await },
        )
        .await?;
        Ok(stream)
    }
}
