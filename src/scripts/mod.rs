#![allow(clippy::module_name_repetitions)]

use crate::input::ScriptsRequired;
use anyhow::{anyhow, Result};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::net::IpAddr;
use std::path::PathBuf;
use std::string::ToString;
use subprocess::{Exec, ExitStatus};
use text_placeholder::Template;

static DEFAULT: &str = r#"tags = ["core_approved", "RustScan", "default"]
developer = [ "RustScan", "https://github.com/RustScan" ]
ports_separator = ","
call_format = "nmap -vvv -p {{port}} {{ip}}"
"#;

#[cfg(not(tarpaulin_include))]
pub fn init_scripts(scripts: ScriptsRequired) -> Result<Vec<ScriptFile>> {
    let mut scripts_to_run: Vec<ScriptFile> = Vec::new();

    match scripts {
        ScriptsRequired::None => Ok(scripts_to_run),
        ScriptsRequired::Default => {
            let default_script =
                toml::from_str::<ScriptFile>(DEFAULT).expect("Failed to parse Script file.");
            scripts_to_run.push(default_script);
            Ok(scripts_to_run)
        }
        ScriptsRequired::Custom => {
            let Some(scripts_dir_base) = dirs::home_dir() else {
                return Err(anyhow!("Could not infer scripts path."));
            };
            let script_paths = match find_scripts(scripts_dir_base) {
                Ok(script_paths) => script_paths,
                Err(e) => return Err(anyhow!(e)),
            };
            println!("Scripts paths \n{:?}", script_paths);

            let parsed_scripts = parse_scripts(script_paths);
            println!("Scripts parsed \n{:?}", parsed_scripts);

            let script_config = match ScriptConfig::read_config() {
                Ok(script_config) => script_config,
                Err(e) => return Err(anyhow!(e)),
            };

            if script_config.tags.is_some() {
                let config_hashset: HashSet<String> =
                    script_config.tags.unwrap().into_iter().collect();
                for script in &parsed_scripts {
                    if script.tags.is_some() {
                        let script_hashset: HashSet<String> =
                            script.tags.clone().unwrap().into_iter().collect();
                        if config_hashset.is_subset(&script_hashset) {
                            scripts_to_run.push(script.clone());
                        } else {
                            println!(
                                "\nScript tags does not match config tags {:?} {}",
                                &script_hashset,
                                script.path.clone().unwrap().display()
                            );
                        }
                    }
                }
            }
            println!("\nScript(s) to run {:?}", scripts_to_run);
            Ok(scripts_to_run)
        }
    }
}

pub fn parse_scripts(scripts: Vec<PathBuf>) -> Vec<ScriptFile> {
    let mut parsed_scripts: Vec<ScriptFile> = Vec::with_capacity(scripts.len());
    for script in scripts {
        println!("Parsing script {}", &script.display());
        if let Some(script_file) = ScriptFile::new(script) {
            parsed_scripts.push(script_file);
        }
    }
    parsed_scripts
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
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

    // Tags found in ScriptFile.
    tags: Option<Vec<String>>,

    // The format how we want the script to run.
    call_format: Option<String>,
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
        tags: Option<Vec<String>>,
        call_format: Option<String>,
    ) -> Self {
        Self {
            path,
            ip,
            open_ports,
            trigger_port,
            ports_separator,
            tags,
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
        if let Some(call_format) = self.call_format {
            final_call_format = call_format;
        } else {
            return Err(anyhow!("Failed to parse execution format."));
        }
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

pub fn find_scripts(mut path: PathBuf) -> Result<Vec<PathBuf>> {
    path.push(".rustscan_scripts");
    if path.is_dir() {
        println!("Scripts folder found {}", &path.display());
        let mut files_vec: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            files_vec.push(entry.path());
        }
        Ok(files_vec)
    } else {
        Err(anyhow!("Can't find scripts folder {}", path.display()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptFile {
    pub path: Option<PathBuf>,
    pub tags: Option<Vec<String>>,
    pub developer: Option<Vec<String>>,
    pub port: Option<String>,
    pub ports_separator: Option<String>,
    pub call_format: Option<String>,
}

impl ScriptFile {
    fn new(script: PathBuf) -> Option<ScriptFile> {
        let real_path = script.clone();
        let mut lines_buf = String::new();
        if let Ok(file) = File::open(script) {
            for mut line in io::BufReader::new(file).lines().skip(1).flatten() {
                if line.starts_with('#') {
                    line.retain(|c| c != '#');
                    line = line.trim().to_string();
                    line.push('\n');
                    lines_buf.push_str(&line);
                } else {
                    break;
                }
            }
        } else {
            println!("Failed to read file: {}", &real_path.display());
            return None;
        }
        println!("ScriptFile {} lines\n{}", &real_path.display(), &lines_buf);

        match toml::from_str::<ScriptFile>(&lines_buf) {
            Ok(mut parsed) => {
                println!("Parsed ScriptFile{} \n{:?}", &real_path.display(), &parsed);
                parsed.path = Some(real_path);
                // parsed_scripts.push(parsed);
                Some(parsed)
            }
            Err(e) => {
                println!("Failed to parse ScriptFile headers {}", e.to_string());
                None
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScriptConfig {
    pub tags: Option<Vec<String>>,
    pub ports: Option<Vec<String>>,
    pub developer: Option<Vec<String>>,
}

#[cfg(not(tarpaulin_include))]
impl ScriptConfig {
    pub fn read_config() -> Result<ScriptConfig> {
        let Some(mut home_dir) = dirs::home_dir() else {
            return Err(anyhow!("Could not infer ScriptConfig path."));
        };
        home_dir.push(".rustscan_scripts.toml");

        let content = fs::read_to_string(home_dir)?;
        let config = toml::from_str::<ScriptConfig>(&content)?;
        Ok(config)
    }
}
