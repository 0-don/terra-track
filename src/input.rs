use std::path::PathBuf;

use serde_derive::Deserialize;

const LOWEST_PORT_NUMBER: u16 = 1;
const TOP_PORT_NUMBER: u16 = 65535;

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
    pub ports: Option<Vec<u16>>,

    /// A range of ports with format start-end. Example: 1-1000.
    pub range: Option<PortRange>,

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
        let mut opts = Opts {
            addresses: vec!["scanme.nmap.org".into()],
            ports: None,
            range: None,
            no_config: false,
            config_path: None,
            greppable: false,
            accessible: false,
            batch_size: 4500,
            timeout: 1000,
            tries: 1,
            ulimit: None,
            scan_order: ScanOrder::Serial,
            scripts: ScriptsRequired::Default,
            top: false,
            command: vec![
                // "-sS".into(),
                "-T2".into(),
                "-n".into(),
                "-vv".into(),
                "-sV".into(),
                "-Pn".into(),
                // "-O".into(),
                "-oX".into(),
                "./nmap.xml".into(),
                "--unprivileged".into(),
            ],
        };

        if opts.ports.is_none() && opts.range.is_none() {
            opts.range = Some(PortRange {
                start: LOWEST_PORT_NUMBER,
                end: TOP_PORT_NUMBER,
            });
        }

        opts
    }
}
