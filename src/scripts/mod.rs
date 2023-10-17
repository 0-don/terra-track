use anyhow::{anyhow, Result};
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::Command;

pub struct Script {
    ip: IpAddr,
    open_ports: Vec<u16>,
}

impl Script {
    pub fn build(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self { ip, open_ports }
    }

    #[allow(unused_assignments)]
    pub fn run(self) -> Result<String> {
        let ports_str = self
            .open_ports
            .iter()
            .map(|port| port.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let to_run = format!(
            "nmap -vvv -p {} {} -T2 -n -vv -sV -Pn -oX ./nmap.xml --unprivileged",
            ports_str, self.ip
        );

        let arguments: Vec<String> = to_run.split_whitespace().map(String::from).collect();

        execute_script(arguments)
    }
}

fn execute_script(arguments: Vec<String>) -> Result<String> {
    let mut iter = arguments.iter();
    let command = iter.next().expect("No command provided");
    let process = Command::new(command).args(iter).output()?;

    if process.status.success() {
        Ok(String::from_utf8_lossy(&process.stdout).into_owned())
    } else {
        Err(anyhow!(
            "Exit code = {}",
            process.status.code().unwrap_or(-1)
        ))
    }
}

#[derive(Debug, Clone)]
pub struct ScriptFile {
    pub path: Option<PathBuf>,
    pub port: Option<String>,
    pub ports_separator: Option<String>,
    pub call_format: Option<String>,
}
