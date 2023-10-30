use std::net::Ipv4Addr;

// LCG parameters determenistic random number generator
pub const A: u32 = 1664525;
pub const C: u32 = 1013904223;
pub const U32_MAX: u32 = std::u32::MAX;
pub const BATCH_SIZE: u32 = U32_MAX;

pub struct Ipv4Iter {
    current: u32,
    batch_size: u32,
    count: u32,
}

impl Ipv4Iter {
    pub fn new(cursor: &str) -> Self {
        let ip = cursor
            .parse::<Ipv4Addr>()
            .expect("Invalid IP address provided");
        Self {
            current: u32::from_be_bytes(ip.octets()),
            batch_size: BATCH_SIZE,
            count: 0,
        }
    }

    pub fn batched(cursor: &str, batch_size: u32) -> Self {
        let ip = cursor
            .parse::<Ipv4Addr>()
            .expect("Invalid IP address provided");
        Self {
            current: u32::from_be_bytes(ip.octets()),
            batch_size,
            count: 0,
        }
    }

    #[inline]
    fn is_reserved(&self, ip_as_u32: u32) -> bool {
        // Binary search on the flattened boundaries
        let pos = match RESERVED_BOUNDARIES.binary_search(&ip_as_u32) {
            Ok(_) => return true, // Exact match means it's a boundary, hence reserved
            Err(pos) => pos,      // Position where it would be inserted
        };

        // Check if the position is odd, which means the IP is within a range
        pos % 2 != 0
    }

    pub fn skip_batch(&mut self, batches_to_skip: u32) -> Option<Ipv4Addr> {
        let total_to_skip = batches_to_skip * self.batch_size;
        let mut last_ip: Option<Ipv4Addr> = None;

        for _ in 0..total_to_skip {
            last_ip = self.next();
        }

        last_ip
    }

    fn next_ip(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.current = (A.wrapping_mul(self.current).wrapping_add(C)) & U32_MAX;
        }
        #[cfg(not(debug_assertions))]
        {
            self.current = (A * self.current + C) & u32::MAX;
        }
    }
}

impl Iterator for Ipv4Iter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Ipv4Addr> {
        if self.count >= self.batch_size {
            return None;
        }

        while self.is_reserved(self.current) {
            self.next_ip();
        }

        let ip = Ipv4Addr::from(self.current);
        self.count += 1;
        self.next_ip();

        Some(ip)
    }
}

#[rustfmt::skip]
pub const RESERVED_BOUNDARIES: [u32; 30] = [
    // (0.0.0.0, 0.255.255.255)
    u32::from_be_bytes(Ipv4Addr::new(0, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(0, 255, 255, 255).octets()),

    // (10.0.0.0, 10.255.255.255)
    u32::from_be_bytes(Ipv4Addr::new(10, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(10, 255, 255, 255).octets()),

    // (100.64.0.0, 100.127.255.255)
    u32::from_be_bytes(Ipv4Addr::new(100, 64, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(100, 127, 255, 255).octets()),

    // (127.0.0.0, 127.255.255.255)
    u32::from_be_bytes(Ipv4Addr::new(127, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(127, 255, 255, 255).octets()),

    // (169.254.0.0, 169.254.255.255)
    u32::from_be_bytes(Ipv4Addr::new(169, 254, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(169, 254, 255, 255).octets()),

    // (172.16.0.0, 172.31.255.255)
    u32::from_be_bytes(Ipv4Addr::new(172, 16, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(172, 31, 255, 255).octets()),

    // (192.0.0.0, 192.0.0.255)
    u32::from_be_bytes(Ipv4Addr::new(192, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(192, 0, 0, 255).octets()),

    // (192.0.2.0, 192.0.2.255)
    u32::from_be_bytes(Ipv4Addr::new(192, 0, 2, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(192, 0, 2, 255).octets()),

    // (192.88.99.0, 192.88.99.255)
    u32::from_be_bytes(Ipv4Addr::new(192, 88, 99, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(192, 88, 99, 255).octets()),

    // (192.168.0.0, 192.168.255.255)
    u32::from_be_bytes(Ipv4Addr::new(192, 168, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(192, 168, 255, 255).octets()),

    // (198.18.0.0, 198.19.255.255)
    u32::from_be_bytes(Ipv4Addr::new(198, 18, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(198, 19, 255, 255).octets()),

    // (198.51.100.0, 198.51.100.255)
    u32::from_be_bytes(Ipv4Addr::new(198, 51, 100, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(198, 51, 100, 255).octets()),

    // (203.0.113.0, 203.0.113.255)
    u32::from_be_bytes(Ipv4Addr::new(203, 0, 113, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(203, 0, 113, 255).octets()),

    // (224.0.0.0, 239.255.255.255)
    u32::from_be_bytes(Ipv4Addr::new(224, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(239, 255, 255, 255).octets()),

    // (240.0.0.0, 255.255.255.255)
    u32::from_be_bytes(Ipv4Addr::new(240, 0, 0, 0).octets()),
    u32::from_be_bytes(Ipv4Addr::new(255, 255, 255, 255).octets()),
];
