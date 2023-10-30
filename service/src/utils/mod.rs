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
