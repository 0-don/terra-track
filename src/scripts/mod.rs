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
    // Path to the script itself.
    path: Option<PathBuf>,

    // Ip got from scanner.
    ip: IpAddr,

    // Ports found with portscan.
    open_ports: Vec<u16>,

    // Port found in ScriptFile, if defined only this will run with the ip.
    trigger_port: Option<String>,

    // Character to join ports in case we want to use a string format of them, for example nmap -p.
    ports_separator: Option<String>,

    // The format how we want the script to run.
    call_format: String,
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
    pub fn build(
        path: Option<PathBuf>,
        ip: IpAddr,
        open_ports: Vec<u16>,
        trigger_port: Option<String>,
        ports_separator: Option<String>,
        call_format: String,
    ) -> Self {
        Self {
            path,
            ip,
            open_ports,
            trigger_port,
            ports_separator,
            call_format,
        }
    }

    // Some variables get changed before read, and compiler throws warning on warn(unused_assignments)
    #[allow(unused_assignments)]
    pub fn run(self) -> Result<String> {
        let separator = self.ports_separator.unwrap_or_else(|| ",".into());

        let mut ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(&separator);
        if let Some(port) = self.trigger_port {
            ports_str = port;
        }

        let mut final_call_format = String::new();

        final_call_format = self.call_format;

        let default_template: Template = Template::new(&final_call_format);
        let mut to_run = String::new();

        if final_call_format.contains("{{script}}") {
            let exec_parts_script: ExecPartsScript = ExecPartsScript {
                script: self.path.unwrap().to_str().unwrap().to_string(),
                ip: self.ip.to_string(),
                port: ports_str,
            };
            to_run = default_template.fill_with_struct(&exec_parts_script)?;
        } else {
            let exec_parts: ExecParts = ExecParts {
                ip: self.ip.to_string(),
                port: ports_str,
            };
            to_run = default_template.fill_with_struct(&exec_parts)?;
        }

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
