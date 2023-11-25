use crate::types::Nmap;
use quickxml_to_serde::{xml_string_to_json, Config};
use std::fs::{create_dir_all, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::IpAddr;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct NmapScanner {
    ip: IpAddr,
    open_ports: Vec<u16>,
    xml_path: String,
    xml_file_path: String,
    xml_nmap_path: String,
}

impl NmapScanner {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        let xml_path = format!("./output/{}", ip.to_string());
        let xml_file_path = format!("./output/{}/{}.xml", ip.to_string(), ip.to_string());
        let xml_nmap_path = format!("./output/{}/{}", ip.to_string(), ip.to_string());
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

        // let udp_ports_str = "7,9,17,19,49,53,67-69,80,88,111,120,123,135-139,158,161-162,177,427,443,445,497,500,514-515,518,520,593,623,626,631,996-999,1022-1023,1025-1030,1433-1434,1645-1646,1701,1718-1719,1812-1813,1900,2000,2048-2049,2222-2223,3283,3456,3703,4444,4500,5000,5060,5353,5632,9200,10000,17185,20031,30718,31337,32768-32769,32771,32815,33281,49152-49154,49156,49181-49182,49185-49186,49188,49190-49194,49200-49201,65024";
        let udp_ports_str = "";

        let full_ports_str = format!("T:{},U:{}", tcp_ports_str, udp_ports_str);
        let ip = self.ip.to_string();

        let scripts = format!(
            "{}",
            vec![
                // CATEGORIES
                format!("(default or version or discovery or auth or vuln or external or exploit or malware or safe or intrusive)"),
                "and not (broadcast-* or targets-asn or http-robtex-shared-ns or lltd-discovery or *multicast* or http-icloud-* or hostmap-robtex or http-virustotal)".to_string(),
                "and not (*dns* or tor-consensus-checker or *domain* or asn-query)".to_string(),
                "and not (http-google-malware or ip-geolocation-map-google or ip-geolocation-map-bing or qscan or http-useragent-tester or http-mobileversion-checker or *slowloris* or *enum*)".to_string(),
                "and not (http-chrono or eap-info or port-states)".to_string(),
            ]
            .join(" "),
        );

        let arguments = vec![
            "nmap",
            "-v6",
            "-d1",
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
            "-p",
            &full_ports_str,
            "--script",
            &scripts,
            "--script-args",
            "http.max-cache-size=2000000",
            &ip,
        ];

        println!("{:?}", arguments.join(" "));

        self.create_directory()?;

        let script = self.execute_script(arguments);
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
                Ok(nmap) => match nmap.nmaprun.host.address.addr == self.ip.to_string() {
                    true => Ok(nmap),
                    false => Err(anyhow::anyhow!("IP does not match")),
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
                xml_text_node_prop_name: "value".to_string(),
                ..Default::default()
            },
        )?
        .to_string();

        File::create(self.xml_file_path.replace(".xml", ".json"))?.write_all(json.as_bytes())?;

        let deserializer = &mut serde_json::Deserializer::from_str(&json);

        let nmap: Result<Nmap, _> = serde_path_to_error::deserialize(deserializer);
        match nmap {
            Ok(n) => Ok(n),
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
