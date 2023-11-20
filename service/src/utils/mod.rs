use std::collections::HashMap;

use chrono::{DateTime, Duration, FixedOffset};
use regex::Regex;

pub fn convert_ipv4_string_to_i32(ipv4_string: &str) -> i32 {
    let octets: Vec<&str> = ipv4_string.split('.').collect();
    let mut result = 0;

    for (i, octet) in octets.iter().enumerate() {
        let octet_value: i32 = octet.parse().unwrap();
        result |= octet_value << ((3 - i) * 8);
    }

    result
}

pub fn convert_i32_to_ipv4_string(ipv4_int: i32) -> String {
    let octets: Vec<String> = (0..4)
        .map(|i| ((ipv4_int >> ((3 - i) * 8)) & 0xFF).to_string())
        .collect();

    octets.join(".")
}

pub fn date(duration: Duration) -> DateTime<FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()) - duration
}

pub fn parse_os_from_nmap_output(nmap_output: &Option<String>) -> (Option<String>, Option<String>) {
    if nmap_output.is_none() {
        return (None, None);
    }
    let os_patterns = vec![
        r"windows\s(server\s)?(11|10|8\.1|8|7|xp)|windows\s\d+-x86_64|linux-gnueabihf-armv\d+",
        r"linux|ubuntu|debian|centos|fedora|red\s?hat|suse|arch\s?linux|manjaro|mint|aix|hp-ux|solaris|bsd|sunos|gnu|vmware|xen|kvm|mac\sos\sx|macos\s(catalina|big\s?sur|monterey|sierra|high\s?sierra|mojave)|android|ios|windows\sphone",
    ];
    let cpu_patterns = vec![
        r"x86_64|x86|i[3579]|ryzen|aarch64|armv\d+|mips\d+|sparc|ppc64|s390x|itanium|alpha|pa-risc|avr|pic|msp430",
    ];
    let mut os_counts = HashMap::new();
    let mut cpu_counts = HashMap::new();
    for pattern in os_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.as_ref().unwrap().lines() {
            if let Some(cap) = re.captures(line) {
                *os_counts.entry(cap[0].to_string()).or_insert(0) += 1;
            }
        }
    }
    for pattern in cpu_patterns {
        let re = Regex::new(pattern).unwrap();
        for line in nmap_output.as_ref().unwrap().lines() {
            if let Some(cap) = re.captures(line) {
                *cpu_counts.entry(cap[0].to_string()).or_insert(0) += 1;
            }
        }
    }
    let os_type = os_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(match_str, _)| match_str);
    let cpu_arch = cpu_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(match_str, _)| match_str);
    (os_type, cpu_arch)
}
