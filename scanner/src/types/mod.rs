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
    pub host: Host,
    pub scaninfo: Vec<Scaninfo>,
    pub scanner: String,
    pub start: i64,
    pub startstr: String,
    pub version: f64,
    pub xmloutputversion: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub address: Address,
    pub endtime: i64,
    pub hostnames: Hostnames,
    pub ipidsequence: Ipidsequence,
    pub os: Os,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Status,
    pub tcpsequence: Tcpsequence,
    pub tcptssequence: Tcptssequence,
    pub times: Times,
    pub trace: Trace,
    pub uptime: Uptime,
    pub hostscript: Option<Hostscript>,
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
pub struct Ipidsequence {
    pub class: String,
    pub values: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Os {
    pub osfingerprint: Osfingerprint,
    pub osmatch: Value,
    pub portused: Portused,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Osfingerprint {
    pub fingerprint: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portused {
    pub portid: i64,
    pub proto: String,
    pub state: String,
}

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
    pub cpe: Option<String>,
    pub method: String,
    pub name: String,
    pub product: Option<String>,
    pub tunnel: Option<String>,
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
pub struct Tcpsequence {
    pub difficulty: String,
    pub index: i64,
    pub values: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tcptssequence {
    pub class: String,
    pub values: String,
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
pub struct Trace {
    pub hop: Hop,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hop {
    pub ipaddr: String,
    pub rtt: f64,
    pub ttl: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uptime {
    pub lastboot: String,
    pub seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hostscript {
    pub script: Vec<Script>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    #[serde(default)]
    pub elem: Vec<Value>,
    pub id: String,
    pub output: String,
    pub table: Option<Table>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub elem: Vec<Elem>,
    pub key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elem {
    pub key: String,
    pub value: Value,
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
