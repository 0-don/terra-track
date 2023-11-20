use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NmapXML {
    pub scanner: String,
    pub args: String,
    pub start: String,
    pub startstr: String,
    pub version: String,
    pub xmloutputversion: String,
    pub scaninfo: ScanInfo,
    pub verbose: Verbose,
    pub debugging: Debugging,
    pub tasks: Option<Vec<Task>>,
    pub host: Vec<Host>,
    pub runstats: RunStats,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScanInfo {
    #[serde(rename = "type")]
    pub scan_type: String,
    pub protocol: String,
    pub numservices: String,
    pub services: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Verbose {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Debugging {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub task: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Host {
    pub starttime: String,
    pub endtime: String,
    pub status: Status,
    pub address: Vec<Address>,
    pub hostnames: Option<Hostnames>,
    pub ports: Ports,
    pub times: Times,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Status {
    pub state: String,
    pub reason: String,
    pub reason_ttl: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub addr: String,
    pub addrtype: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hostnames {
    #[serde(default)]
    pub hostname: Vec<Hostname>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hostname {
    pub name: String,
    #[serde(rename = "type")]
    pub hostname_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ports {
    #[serde(default)]
    pub port: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Port {
    pub protocol: String,
    pub portid: String,
    pub state: Option<State>,
    pub service: Service,
    #[serde(default)]
    pub cpe: Vec<String>,
    #[serde(default)]
    pub script: Vec<Script>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Script {
    pub id: String,
    pub output: String,
    #[serde(default)]
    pub table: Vec<Table>,
    #[serde(default)]
    pub elem: Vec<Elem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub key: Option<String>,
    #[serde(default)]
    pub elem: Vec<Elem>,
    #[serde(default = "Vec::new")]
    pub table: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Elem {
    pub key: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub state: String,
    pub reason: String,
    pub reason_ttl: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Service {
    pub name: String,
    pub product: Option<String>,
    pub servicefp: Option<String>,
    pub version: Option<String>,
    pub extrainfo: Option<String>,
    pub ostype: Option<String>,
    pub method: String,
    pub conf: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Times {
    pub srtt: String,
    pub rttvar: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RunStats {
    pub finished: Finished,
    pub hosts: Hosts,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Finished {
    pub time: String,
    pub timestr: String,
    pub elapsed: String,
    pub summary: String,
    pub exit: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hosts {
    pub up: String,
    pub down: String,
    pub total: String,
}

// fn deserialize_elems<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let elems_array: Vec<Elem> = Deserialize::deserialize(deserializer)?;
//     let elems_map = elems_array
//         .into_iter()
//         .filter_map(|elem| match (elem.key, elem.value) {
//             (Some(key), Some(value)) => Some((key, value)),
//             (Some(key), None) => Some((key.clone(), key)),
//             (None, Some(value)) => Some((value.clone(), value)),
//             (None, None) => None,
//         })
//         .collect();
//     Ok(elems_map)
// }
