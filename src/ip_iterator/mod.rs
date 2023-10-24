use std::net::Ipv4Addr;
use sha2::{Sha256, Digest};

#[rustfmt::skip]
const RESERVED_RANGES: [(Ipv4Addr, Ipv4Addr); 15] = [
    (Ipv4Addr::new(0, 0, 0, 0), Ipv4Addr::new(0, 255, 255, 255)),
    (Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 255, 255, 255)),
    (Ipv4Addr::new(100, 64, 0, 0), Ipv4Addr::new(100, 127, 255, 255)),
    (Ipv4Addr::new(127, 0, 0, 0), Ipv4Addr::new(127, 255, 255, 255)),
    (Ipv4Addr::new(169, 254, 0, 0), Ipv4Addr::new(169, 254, 255, 255)),
    (Ipv4Addr::new(172, 16, 0, 0), Ipv4Addr::new(172, 31, 255, 255)),
    (Ipv4Addr::new(192, 0, 0, 0), Ipv4Addr::new(192, 0, 0, 255)),
    (Ipv4Addr::new(192, 0, 2, 0), Ipv4Addr::new(192, 0, 2, 255)),
    (Ipv4Addr::new(192, 88, 99, 0), Ipv4Addr::new(192, 88, 99, 255)),
    (Ipv4Addr::new(192, 168, 0, 0), Ipv4Addr::new(192, 168, 255, 255)),
    (Ipv4Addr::new(198, 18, 0, 0), Ipv4Addr::new(198, 19, 255, 255)),
    (Ipv4Addr::new(198, 51, 100, 0), Ipv4Addr::new(198, 51, 100, 255)),
    (Ipv4Addr::new(203, 0, 113, 0), Ipv4Addr::new(203, 0, 113, 255)),
    (Ipv4Addr::new(224, 0, 0, 0), Ipv4Addr::new(239, 255, 255, 255)),
    (Ipv4Addr::new(240, 0, 0, 0), Ipv4Addr::new(255, 255, 255, 255)),
];
pub struct Ipv4Iter {
    current: u32,
    offset: u32,
}

impl Ipv4Iter {
    pub fn new(cursor: &str, offset: u32) -> Self {
        let ip = cursor
            .parse::<Ipv4Addr>()
            .expect("Invalid IP address provided");
        let current = u32::from_be_bytes(ip.octets());

        Self { current, offset }
    }

    fn is_reserved(&self, ip: &Ipv4Addr) -> bool {
        RESERVED_RANGES
            .iter()
            .any(|&(start, end)| *ip >= start && *ip <= end)
    }

    fn permute_ip(&self, ip: u32) -> u32 {
        let hash = Sha256::digest(&ip.to_be_bytes());
        u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
    }

    fn next_ip(&mut self) {
        // Increment by the offset
        self.current = self.current.wrapping_add(self.offset);
    }

    fn to_ipv4(&self, num: u32) -> Ipv4Addr {
        Ipv4Addr::new(
            ((num >> 24) & 0xFF) as u8,
            ((num >> 16) & 0xFF) as u8,
            ((num >> 8) & 0xFF) as u8,
            (num & 0xFF) as u8,
        )
    }
}

impl Iterator for Ipv4Iter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Ipv4Addr> {
        let mut ip = self.to_ipv4(self.permute_ip(self.current));
        while self.is_reserved(&ip) {
            self.next_ip();
            ip = self.to_ipv4(self.permute_ip(self.current));
        }
        // Increment the current IP by the offset after confirming it's not reserved
        self.next_ip();
        Some(ip)
    }
}
