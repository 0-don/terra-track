#![allow(clippy::module_name_repetitions)]

use anyhow::{anyhow, Result};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryInto;
use std::net::IpAddr;
use std::path::PathBuf;
use std::string::ToString;
use subprocess::{Exec, ExitStatus};
use text_placeholder::Template;

pub struct Script {
    // Ip got from scanner.
    ip: IpAddr,

    // Ports found with portscan.
    open_ports: Vec<u16>,
}

#[derive(Serialize)]
struct ExecPartsScript {
    script: String,
    ip: String,
    port: String,
}

#[derive(Serialize)]
struct ExecParts {
    ip: String,
    port: String,
}

impl Script {
    pub fn build(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self { ip, open_ports }
    }

    // Some variables get changed before read, and compiler throws warning on warn(unused_assignments)
    #[allow(unused_assignments)]
    pub fn run(self) -> Result<String> {
        let separator = ",".to_string();

        let ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(&separator);

        let final_call_format =
            "nmap -vvv -p {{port}} {{ip}} -T2 -n -vv -sV -Pn -oX ./nmap.xml --unprivileged"
                .to_string();

        let default_template: Template = Template::new(&final_call_format);
        let mut to_run = String::new();

        let exec_parts: ExecParts = ExecParts {
            ip: self.ip.to_string(),
            port: ports_str,
        };
        to_run = default_template.fill_with_struct(&exec_parts)?;

        let arguments = shell_words::split(&to_run).expect("Failed to parse script arguments");

        execute_script(arguments)
    }
}

#[cfg(not(tarpaulin_include))]
fn execute_script(mut arguments: Vec<String>) -> Result<String> {
    println!("\nScript arguments vec: {:?}", &arguments);
    let process = Exec::cmd(arguments.remove(0)).args(&arguments);
    match process.capture() {
        Ok(c) => {
            let es = match c.exit_status {
                ExitStatus::Exited(c) => c.try_into().unwrap(),
                ExitStatus::Signaled(c) => c.into(),
                ExitStatus::Other(c) => c,
                ExitStatus::Undetermined => -1,
            };
            if es != 0 {
                return Err(anyhow!("Exit code = {}", es));
            }
            Ok(c.stdout_str())
        }
        Err(error) => {
            println!("Command error {}", error.to_string());
            Err(anyhow!(error.to_string()))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptFile {
    pub path: Option<PathBuf>,

    pub port: Option<String>,
    pub ports_separator: Option<String>,
    pub call_format: Option<String>,
}
