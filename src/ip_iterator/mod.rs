use std::net::Ipv4Addr;

// LCG parameters
pub const A: u32 = 1664525;
pub const C: u32 = 1013904223;
pub const BATCH_SIZE: u32 = 1000000;
pub const U32_MAX: u32 = std::u32::MAX;

pub struct Ipv4Iter {
    current: u32,
    batch_size: u32,
    count: u32,
}

impl Ipv4Iter {
    pub fn new(cursor: &str) -> Self {
        let ip = cursor.parse::<Ipv4Addr>().expect("Invalid IP address provided");
        Self {
            current: u32::from_be_bytes(ip.octets()),
            batch_size: BATCH_SIZE,
            count: 0,
        }
    }

    #[inline]
    fn is_reserved(&self, ip_as_u32: u32) -> bool {
        for &(start, end) in RESERVED_RANGES.iter() {
            if ip_as_u32 >= start && ip_as_u32 <= end {
                return true;
            }
        }
        false
    }

    fn next_ip(&mut self) {
        self.current = (A.wrapping_mul(self.current).wrapping_add(C)) & U32_MAX;
    }
}

impl Iterator for Ipv4Iter {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Ipv4Addr> {
        if self.count >= self.batch_size {
            return None;
        }
        
        let mut ip;
        
        loop {
            ip = Ipv4Addr::from(self.current);
            self.next_ip();

            if !self.is_reserved(u32::from_be_bytes(ip.octets())) {
                break;
            }
        }

        self.count += 1;
        Some(ip)
    }
}

macro_rules! ip_to_u32 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        u32::from_be_bytes(Ipv4Addr::new($a, $b, $c, $d).octets())
    };
}

pub const RESERVED_RANGES: [(u32, u32); 15] = [
    (ip_to_u32!(0, 0, 0, 0), ip_to_u32!(0, 255, 255, 255)),
    (ip_to_u32!(10, 0, 0, 0), ip_to_u32!(10, 255, 255, 255)),
    (ip_to_u32!(100, 64, 0, 0), ip_to_u32!(100, 127, 255, 255)),
    (ip_to_u32!(127, 0, 0, 0), ip_to_u32!(127, 255, 255, 255)),
    (ip_to_u32!(169, 254, 0, 0), ip_to_u32!(169, 254, 255, 255)),
    (ip_to_u32!(172, 16, 0, 0), ip_to_u32!(172, 31, 255, 255)),
    (ip_to_u32!(192, 0, 0, 0), ip_to_u32!(192, 0, 0, 255)),
    (ip_to_u32!(192, 0, 2, 0), ip_to_u32!(192, 0, 2, 255)),
    (ip_to_u32!(192, 88, 99, 0), ip_to_u32!(192, 88, 99, 255)),
    (ip_to_u32!(192, 168, 0, 0), ip_to_u32!(192, 168, 255, 255)),
    (ip_to_u32!(198, 18, 0, 0), ip_to_u32!(198, 19, 255, 255)),
    (ip_to_u32!(198, 51, 100, 0), ip_to_u32!(198, 51, 100, 255)),
    (ip_to_u32!(203, 0, 113, 0), ip_to_u32!(203, 0, 113, 255)),
    (ip_to_u32!(224, 0, 0, 0), ip_to_u32!(239, 255, 255, 255)),
    (ip_to_u32!(240, 0, 0, 0), ip_to_u32!(255, 255, 255, 255)),
];
