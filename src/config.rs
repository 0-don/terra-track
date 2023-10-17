pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Debug, Clone)]
pub struct Opts {
    /// A comma-delimited list or newline-delimited file of separated CIDRs, IPs, or hosts to be scanned.
    pub addresses: Vec<String>,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    pub batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    pub timeout: u32,

    /// The number of tries before a port is assumed to be closed.
    /// If set to 0, rustscan will correct it to 1.
    pub tries: u8,
}
