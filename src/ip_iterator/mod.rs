use std::net::IpAddr;
use std::net::Ipv4Addr;

use chrono::Utc;

use crate::printlog;

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

pub fn public_ips() -> Vec<IpAddr> {
    // Convert from u32 to Ipv4Addr
    printlog!("Start");

    let to_ipv4 = |num: u32| -> Ipv4Addr {
        Ipv4Addr::new(
            ((num >> 24) & 0xFF) as u8,
            ((num >> 16) & 0xFF) as u8,
            ((num >> 8) & 0xFF) as u8,
            (num & 0xFF) as u8,
        )
    };

    printlog!("created all ips");

    let res: Vec<IpAddr> = (0..=4294967295u32) // Iterate over the entire IPv4 address space as integers
        .map(to_ipv4) // Convert each integer to an Ipv4Addr
        .filter(|&ip| !is_reserved(ip, &RESERVED_RANGES)) // Filter out reserved IPs
        .map(IpAddr::V4)
         // Convert to IpAddr enum variant
        .collect();

    printlog!("finished");

    res
}

pub fn is_reserved(ip: Ipv4Addr, reserved_ranges: &[(Ipv4Addr, Ipv4Addr)]) -> bool {
    reserved_ranges
        .iter()
        .any(|&(start, end)| ip >= start && ip <= end)
}
