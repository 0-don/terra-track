pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Debug, Clone)]
pub struct Opts {
    /// A comma-delimited list or newline-delimited file of separated CIDRs, IPs, or hosts to be scanned.
    pub addresses: Vec<String>,
}
