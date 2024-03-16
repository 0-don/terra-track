use crate::types::Nmap;
use crate::utils::constants::SCRIPTS;
use crate::VALUE;
use quickxml_to_serde::{xml_string_to_json, Config, NullValue};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct NmapScanner {
    ip: IpAddr,
    open_ports: Vec<u16>,
    xml_path: PathBuf,
    xml_file_path: String,
    xml_nmap_path: String,
}

impl NmapScanner {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        let xml_path: PathBuf = env::current_dir()
            .expect("Failed to get current directory")
            .join("output")
            .join(&ip.to_string());
        let xml_file_path = xml_path.join(format!("{ip:?}.xml")).display().to_string();
        let xml_nmap_path = xml_path.join(ip.to_string()).display().to_string();
        Self {
            ip,
            open_ports,
            xml_path,
            xml_file_path,
            xml_nmap_path,
        }
    }

    pub fn run(self) -> anyhow::Result<Nmap> {
        if let Ok(nmap) = self.get_file_if_exist() {
            return Ok(nmap);
        }

        let tcp_ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        let udp_ports_str = "";

        let full_ports_str = format!("T:{},U:{}", tcp_ports_str, udp_ports_str);
        let ip = self.ip.to_string();

        let arguments = [
            "nmap",
            "-v6",
            "-dd",
            "-T4",
            "-n",
            "-A",
            "-Pn",
            "-sV",
            "--version-all",
            "-sC",
            "-O",
            "-oA",
            &self.xml_nmap_path,
            "-sS",
            "-sU",
            "--script-timeout=1m",
            "-p",
            &full_ports_str,
            "--script",
            &SCRIPTS.join(" "),
            "--script-args",
            "http.max-cache-size=2000000",
            &ip,
        ];

        println!("{:?}", arguments.join(" "));

        self.create_directory()?;

        let script = self.execute_script(arguments.to_vec());
        match script {
            Ok(_nmap) => self.parse_nmap_xml(),
            Err(err) => {
                println!("{:?}", err);
                Err(anyhow::anyhow!("Script failed"))
            }
        }
    }

    fn create_directory(&self) -> io::Result<()> {
        create_dir_all(&self.xml_path)
    }

    fn get_file_if_exist(&self) -> anyhow::Result<Nmap> {
        match Path::new(&self.xml_file_path).exists() {
            true => match self.parse_nmap_xml() {
                Ok(nmap) => match &nmap.nmaprun.host {
                    Some(host) => match host.address.addr == self.ip.to_string() {
                        true => Ok(nmap),
                        false => Err(anyhow::anyhow!("IP does not match")),
                    },
                    None => Err(anyhow::anyhow!("Failed to parse XML")),
                },
                Err(_) => Err(anyhow::anyhow!("Failed to parse XML")),
            },
            false => Err(anyhow::anyhow!("File does not exist")),
        }
    }

    fn execute_script(&self, arguments: Vec<&str>) -> anyhow::Result<String> {
        let command = arguments[0];
        let mut child = Command::new(command)
            .args(&arguments[1..])
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;
        let reader = BufReader::new(stdout);

        let mut output = String::new();

        for line_result in reader.lines() {
            let line = line_result?;
            println!("{}", line); // Print each line as it's read
            output.push_str(&line);
            output.push('\n');
        }

        match child.wait() {
            Ok(status) if status.success() => Ok(output),
            Ok(status) => Err(anyhow::anyhow!(
                "Script execution failed with exit code: {}",
                status.code().unwrap_or(-1)
            )),
            Err(e) => Err(anyhow::anyhow!("Failed to wait for child process: {}", e)),
        }
    }

    pub fn parse_nmap_xml(&self) -> anyhow::Result<Nmap> {
        self.create_directory()?;
        let mut file = File::open(&self.xml_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let json = xml_string_to_json(
            contents,
            &Config {
                xml_attr_prefix: "".to_string(),
                xml_text_node_prop_name: VALUE.to_string(),
                empty_element_handling: NullValue::Ignore,
                ..Default::default()
            },
        )?;

        // Serialize the JSON with pretty formatting
        let pretty_json = serde_json::to_string_pretty(&json)?;

        // Write the prettified JSON to a file
        File::create(self.xml_file_path.replace(".xml", ".json"))?
            .write_all(pretty_json.as_bytes())?;

        let deserializer = &mut serde_json::Deserializer::from_str(&pretty_json);

        let nmap: Result<Nmap, _> = serde_path_to_error::deserialize(deserializer);
        match nmap {
            Ok(n) => match &n.nmaprun.host {
                Some(host) => match host.address.addr == self.ip.to_string() {
                    true => Ok(n),
                    false => Err(anyhow::anyhow!("IP does not match")),
                },
                None => Err(anyhow::anyhow!("Failed to parse XML")),
            },
            Err(err) => {
                let path = err.path().to_string();
                let error = err.to_string();
                println!("\n\n{}", path);
                println!("{}\n\n", error);
                panic!();
            }
        }
    }
}
