use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nmap {
    pub nmaprun: Nmaprun,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nmaprun {
    pub args: String,
    pub debugging: Debugging,
    pub host: Host,
    pub runstats: Runstats,
    pub scaninfo: Scaninfo,
    pub scanner: String,
    pub start: i64,
    pub startstr: String,
    pub taskbegin: Vec<Taskbegin>,
    pub taskend: Vec<Taskend>,
    pub taskprogress: Vec<Taskprogress>,
    pub verbose: Verbose,
    pub version: f64,
    pub xmloutputversion: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Debugging {
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub address: Address,
    pub endtime: i64,
    pub hostnames: Hostnames,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Status,
    pub times: Times,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub addr: String,
    pub addrtype: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hostnames {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ports {
    pub port: Vec<Port>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub portid: i64,
    pub protocol: String,
    pub script: Value,
    pub service: Service,
    pub state: State,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub conf: i64,
    pub method: String,
    pub name: String,
    pub product: Option<String>,
    pub servicefp: Option<String>,
    pub tunnel: Option<String>,
    pub cpe: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub reason: String,
    #[serde(rename = "reason_ttl")]
    pub reason_ttl: i64,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub reason: String,
    #[serde(rename = "reason_ttl")]
    pub reason_ttl: i64,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Times {
    pub rttvar: i64,
    pub srtt: i64,
    pub to: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runstats {
    pub finished: Finished,
    pub hosts: Hosts,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finished {
    pub elapsed: f64,
    pub exit: String,
    pub summary: String,
    pub time: i64,
    pub timestr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hosts {
    pub down: i64,
    pub total: i64,
    pub up: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scaninfo {
    pub numservices: i64,
    pub protocol: String,
    pub services: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taskbegin {
    pub task: String,
    pub time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taskend {
    pub task: String,
    pub time: i64,
    pub extrainfo: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taskprogress {
    pub etc: i64,
    pub percent: f64,
    pub remaining: i64,
    pub task: String,
    pub time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verbose {
    pub level: i64,
}
