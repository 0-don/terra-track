use std::path::PathBuf;

use serde_derive::Deserialize;

pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

// CONFIG
pub const PORTS: Option<Vec<u16>> = None;
pub const RANGE: PortRange = PortRange {
    start: LOWEST_PORT_NUMBER,
    end: TOP_PORT_NUMBER,
};
pub const NO_CONFIG: bool = false;
pub const CONFIG_PATH: Option<PathBuf> = None;
pub const GREPPABLE: bool = false;
pub const ACCESSIBLE: bool = false;
pub const BATCH_SIZE: u16 = 4500;
pub const TIMEOUT: u32 = 1000;
pub const TRIES: u8 = 1;
pub const ULIMIT: Option<u64> = None;
pub const SCAN_ORDER: ScanOrder = ScanOrder::Serial;
pub const SCRIPTS: ScriptsRequired = ScriptsRequired::Default;
pub const TOP: bool = false;
pub const COMMAND: [&str; 8] = [
    "-T2",
    "-n",
    "-vv",
    "-sV",
    "-Pn",
    "-oX",
    "./nmap.xml",
    "--unprivileged",
];

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ScanOrder {
    Serial,
    Random,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ScriptsRequired {
    None,
    Default,
    Custom,
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

    /// A list of comma separated ports to be scanned. Example: 80,443,8080.
    // pub ports: Option<Vec<u16>>,

    // /// A range of ports with format start-end. Example: 1-1000.
    // pub range: PortRange,

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

    /// The Script arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -T 1500 -a 127.0.0.1 -- -A -sC'.
    /// This command adds -Pn -vvv -p $PORTS automatically to nmap.
    /// For things like --script '(safe and vuln)' enclose it in quotations marks \"'(safe and vuln)'\"")
    pub command: Vec<String>,
}

impl Opts {
    pub fn read() -> Self {
        Opts {
            addresses: vec!["scanme.nmap.org".into()],
            // ports: PORTS,
            // range: RANGE,
            no_config: NO_CONFIG,
            config_path: CONFIG_PATH,
            greppable: GREPPABLE,
            accessible: ACCESSIBLE,
            batch_size: BATCH_SIZE,
            timeout: TIMEOUT,
            tries: TRIES,
            ulimit: ULIMIT,
            scan_order: SCAN_ORDER,
            scripts: SCRIPTS,
            top: TOP,
            command: COMMAND.iter().map(|s| s.to_string()).collect(),
        }
    }
}
