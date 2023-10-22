use std::net::IpAddr;
use std::process::Command;

pub struct Script {
    ip: IpAddr,
    open_ports: Vec<u16>,
}

impl Script {
    // A more conventional naming would be `new` instead of `build`
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self { ip, open_ports }
    }

    pub fn run(self) -> anyhow::Result<String> {
        // Format the ports as a comma-separated string
        let ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        // Construct the nmap command
        let to_run = format!(
            "nmap -vvv -p {} {} -T2 -n -sV -Pn -oX ./nmap.xml --unprivileged",
            ports_str, self.ip
        );

        // Split the command string into arguments
        let arguments = to_run.split_whitespace().map(String::from).collect();

        Self::execute_script(arguments)
    }

    fn execute_script(arguments: Vec<String>) -> anyhow::Result<String> {
        let mut iter = arguments.iter();

        // Expect a command to be provided
        let command = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("No command provided"))?;

        // Execute the command with the given arguments
        let process = Command::new(command).args(iter).output()?;

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
