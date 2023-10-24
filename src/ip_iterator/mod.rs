use std::net::Ipv4Addr;

const BATCH_SIZE: u16 = 4500;

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
    batch_size: u16,
    count: u32,
}

impl Ipv4Iter {
    pub fn new(cursor: &str) -> Self {
        let ip = cursor
            .parse::<Ipv4Addr>()
            .expect("Invalid IP address provided");
        let current = u32::from_be_bytes(ip.octets());

        Self {
            current,
            batch_size: BATCH_SIZE,
            count: 0,
        }
    }

    fn is_reserved(&self, ip: &Ipv4Addr) -> bool {
        RESERVED_RANGES
            .iter()
            .any(|&(start, end)| *ip >= start && *ip <= end)
    }

    fn next_ip(&mut self) {
        // LCG parameters
        let a: u32 = 1664525;
        let c: u32 = 1013904223;
        self.current = (a.wrapping_mul(self.current).wrapping_add(c)) & u32::MAX;
    }
}

impl Iterator for Ipv4Iter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Ipv4Addr> {
        if self.count >= self.batch_size as u32 {
            return None;
        }

        let mut ip = Ipv4Addr::from(self.current);
        while self.is_reserved(&ip) {
            self.next_ip();
            ip = Ipv4Addr::from(self.current);
        }

        self.count += 1;
        self.next_ip();
        Some(ip)
    }
}
