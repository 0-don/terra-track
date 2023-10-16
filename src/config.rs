use serde_derive::Deserialize;
use std::path::PathBuf;

pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ScanOrder {
    Serial,
    Random,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ScriptsRequired {
    Default,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

#[derive(Debug, Clone)]
pub struct Opts {
    /// A comma-delimited list or newline-delimited file of separated CIDRs, IPs, or hosts to be scanned.
    pub addresses: Vec<String>,

    /// Whether to ignore the configuration file or not.
    pub no_config: bool,

    /// Custom path to config file
    pub config_path: Option<PathBuf>,

    /// Greppable mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    pub greppable: bool,

    /// Accessible mode. Turns off features which negatively affect screen readers.
    pub accessible: bool,

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

    /// Automatically ups the ULIMIT with the value you provided.
    pub ulimit: Option<u64>,

    /// The order of scanning to be performed. The "serial" option will
    /// scan ports in ascending order while the "random" option will scan
    /// ports randomly.
    pub scan_order: ScanOrder,

    /// Level of scripting required for the run.
    pub scripts: ScriptsRequired,

    /// Use the top 1000 ports.
    pub top: bool,
}
