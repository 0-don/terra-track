use std::net::IpAddr;
use std::process::Command;

pub struct Script {
    ip: IpAddr,
    open_ports: Vec<u16>,
}

impl Script {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self { ip, open_ports }
    }

    pub fn run(self) -> anyhow::Result<String> {
        // Convert ports to string and join with commas
        let ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        // Construct the list of arguments with the IP and ports
        let binding = self.ip.to_string();
        let arguments = vec![
            "nmap",
            "-vvv",
            "-T2",
            "-n",
            "-sV",
            "-Pn",
            "-oX",
            "./nmap.xml",
            "--unprivileged",
            "-p",
            &ports_str,
            &binding,
        ];

        Self::execute_script(arguments)
    }

    fn execute_script(arguments: Vec<&str>) -> anyhow::Result<String> {
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
}
