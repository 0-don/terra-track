use super::PortStrategy;
use crate::port_strategy::SerialRange;
mod socket_iterator;
use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use socket_iterator::SocketIterator;
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
    ips: Vec<IpAddr>,
    batch_size: u16,
    timeout: Duration,
    tries: NonZeroU8,
    port_strategy: PortStrategy,
}

impl Scanner {
    pub fn new(ips: Vec<IpAddr>) -> Self {
        Self {
            batch_size: 4500,
            timeout: Duration::from_millis(1000),
            tries: NonZeroU8::new(std::cmp::max(1, 1)).unwrap(),
            port_strategy: PortStrategy::Serial(SerialRange {
                start: LOWEST_PORT_NUMBER,
                end: TOP_PORT_NUMBER,
            }),
            ips,
        }
    }

    pub async fn run(&self) -> HashMap<IpAddr, Vec<u16>> {
        let ports: Vec<u16> = self.port_strategy.order();
        let mut socket_iterator: SocketIterator = SocketIterator::new(&self.ips, &ports);
        let mut open_sockets: Vec<SocketAddr> = Vec::new();
        let mut ftrs = FuturesUnordered::new();
        let mut errors: HashSet<String> = HashSet::with_capacity(self.ips.len() * 1000);

        for _ in 0..self.batch_size {
            if let Some(socket) = socket_iterator.next() {
                ftrs.push(self.scan_socket(socket));
            } else {
                break;
            }
        }

        while let Some(result) = ftrs.next().await {
            if let Some(socket) = socket_iterator.next() {
                ftrs.push(self.scan_socket(socket));
            }

            match result {
                Ok(socket) => open_sockets.push(socket),
                Err(e) => {
                    let error_string = e.to_string();
                    if errors.len() < self.ips.len() * 1000 {
                        errors.insert(error_string);
                    }
                }
            }
        }

        let mut ports_per_ip: HashMap<IpAddr, Vec<u16>> = HashMap::new();

        for socket in open_sockets {
            ports_per_ip
                .entry(socket.ip())
                .or_insert_with(Vec::new)
                .push(socket.port());
        }

        for ip in &self.ips {
            if ports_per_ip.contains_key(&ip) {
                continue;
            }
            println!("{} is not accessible", ip);
        }

        println!("Open Sockets found: {:?}", &ports_per_ip);

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
