pub fn convert_ipv4_string_to_i32(ipv4_string: &str) -> i32 {
    let ipv4_string = ipv4_string.to_string();
    let ipv4_string = ipv4_string.replace(".", "");
    let ipv4_string = ipv4_string.parse::<i32>().unwrap();
    ipv4_string
}

pub fn convert_i32_to_ipv4_string(ipv4_int: i32) -> String {
    let ipv4_string = ipv4_int.to_string();
    let ipv4_string = ipv4_string.as_bytes();
    let ipv4_string = std::str::from_utf8(ipv4_string).unwrap();
    let ipv4_string = ipv4_string.chars().collect::<Vec<char>>();
    let ipv4_string = ipv4_string
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();
    let ipv4_string = ipv4_string.join(".");
    ipv4_string
}
