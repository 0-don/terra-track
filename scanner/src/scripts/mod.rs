use crate::types::Nmap;
use quickxml_to_serde::{xml_string_to_json, Config};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::IpAddr;
use std::path::Path;
use std::process::Command;

pub struct Script {
    ip: IpAddr,
    open_ports: Vec<u16>,
    xml: String,
}

impl Script {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self {
            ip,
            open_ports,
            xml: format!("./output/{}.xml", ip.to_string()),
        }
    }

    pub fn run(self) -> anyhow::Result<Nmap> {
        if let Ok(nmap) = self.get_file_if_exist() {
            return Ok(nmap);
        }

        // Convert ports to a comma-separated string for TCP
        let tcp_ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        // Fixed list of well-known UDP ports
        let udp_ports_str = "7,9,17,19,49,53,67-69,80,88,111,120,123,135-139,158,161-162,177,427,443,445,497,500,514-515,518,520,593,623,626,631,996-999,1022-1023,1025-1030,1433-1434,1645-1646,1701,1718-1719,1812-1813,1900,2000,2048-2049,2222-2223,3283,3456,3703,4444,4500,5000,5060,5353,5632,9200,10000,17185,20031,30718,31337,32768-32769,32771,32815,33281,49152-49154,49156,49181-49182,49185-49186,49188,49190-49194,49200-49201,65024";

        let full_ports_str = format!("{},{}", tcp_ports_str, udp_ports_str);
        let ip = self.ip.to_string();
        // Construct the nmap arguments
        let arguments = vec![
            "nmap",
            "-v6",           // Verbose
            "-T4",           // Timing template (higher is faster)
            "-n",            // Never do DNS resolution
            "-A",  // Enable OS detection, version detection, script scanning, and traceroute
            "-Pn", // Treat all hosts as online -- skip host discovery
            "-sV", // Probe open ports to determine service/version info
            "--version-all", // Try every single probe
            "-sC", // Script scan using the default set of scripts
            "-O",  // Enable OS detection
            "-oX", // XML output
            self.xml.as_str(),
            "-p",                    // Specify ports
            full_ports_str.as_str(), // Ports
            "--script=vuln",         // Vulnerability scanning
            "-D",
            "RND:10",    // Using 10 random decoys
            ip.as_str(), // Target IP
        ];

        println!("{:?}", arguments.join(" "));

        self.create_directory();

        let script = self.execute_script(arguments);
        match script {
            Ok(_nmap) => self.parse_nmap_xml(),
            Err(err) => {
                println!("{:?}", err);
                Err(anyhow::anyhow!("Script failed"))
            }
        }
    }

    // Separate method to create directory based on the XML path
    fn create_directory(&self) {
        if let Some(parent) = Path::new(&self.xml).parent() {
            if !parent.exists() {
                create_dir_all(parent).expect("Failed to create directory");
            }
        }
    }

    fn get_file_if_exist(&self) -> anyhow::Result<Nmap> {
        if Path::new(&self.xml).exists() {
            let nmap = self.parse_nmap_xml();
            if let Ok(nmap) = nmap {
                if nmap.nmaprun.host.address.addr == self.ip.to_string() {
                    return Ok(nmap);
                }
                println!("IP does not match");
                return Err(anyhow::anyhow!("IP does not match"));
            }
            println!("Failed to parse XML");
            return Err(anyhow::anyhow!("Failed to parse XML"));
        }
        println!("File does not exist");
        Err(anyhow::anyhow!("File does not exist"))
    }

    fn execute_script(&self, arguments: Vec<&str>) -> anyhow::Result<String> {
        // The first argument is always the command
        let command = arguments[0];

        // Execute the command with the given arguments
        let process = Command::new(command).args(&arguments[1..]).output()?;

        // Check if the process was successful and return the output or an error
        if process.status.success() {
            Ok(String::from_utf8_lossy(&process.stdout).into_owned())
        } else {
            Err(anyhow::anyhow!(
                "Exit code = {}",
                process.status.code().unwrap_or(-1)
            ))
        }
    }

    pub fn parse_nmap_xml(&self) -> anyhow::Result<Nmap> {
        self.create_directory();
        let mut file = File::open(self.xml.clone())?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let json = xml_string_to_json(
            contents.clone(),
            &Config {
                xml_attr_prefix: "".to_string(),
                xml_text_node_prop_name: "value".to_string(),
                ..Default::default()
            },
        )?
        .to_string();

        File::create(self.xml.clone().replace(".xml", ".json"))?.write_all(json.as_bytes())?;

        let nmap: Nmap = serde_json::from_str(json.as_str())?;
        Ok(nmap)
    }
}
