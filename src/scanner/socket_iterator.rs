use std::net::{IpAddr, SocketAddr};

pub struct SocketIterator<'s> {
    ips: &'s [IpAddr],
    ports: &'s [u16],
    ip_idx: usize,
    port_idx: usize,
}

impl<'s> SocketIterator<'s> {
    pub fn new(ips: &'s [IpAddr], ports: &'s [u16]) -> Self {
        Self {
            ips,
            ports,
            ip_idx: 0,
            port_idx: 0,
        }
    }
}

impl<'s> Iterator for SocketIterator<'s> {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip_idx < self.ips.len() && self.port_idx < self.ports.len() {
            let result = SocketAddr::new(self.ips[self.ip_idx], self.ports[self.port_idx]);
            self.port_idx += 1;
            if self.port_idx == self.ports.len() {
                self.port_idx = 0;
                self.ip_idx += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}
