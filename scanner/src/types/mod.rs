use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nmap {
    pub nmaprun: Nmaprun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub address: Address,
    pub os: Os,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Stat,
    pub hostscript: Option<Hostscript>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub addr: String,
    pub addrtype: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hostscript {
    pub script: Vec<Script>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elem {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Os {
    pub osfingerprint: Osfingerprint,
    pub osmatch: Option<OsmatchUnion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Osfingerprint {
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsclassElement {
    pub accuracy: i64,
    pub cpe: String,
    pub osfamily: String,
    #[serde(rename = "type")]
    pub osclass_type: Value,
    pub vendor: String,
    pub osgen: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Osmatch {
    pub accuracy: i64,
    pub line: i64,
    pub name: String,
    pub osclass: OsMatchClassUnion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    pub port: Vec<Port>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub portid: i64,
    pub protocol: String,
    pub script: Option<ScriptUnion>,
    pub service: Service,
    pub state: Stat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Script {
    pub id: String,
    pub output: Value,
    pub elem: Option<ElemUnion>,
    pub table: Option<TableUnion>,
    pub value: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub key: String,
    pub elem: Option<ElemUnion>,
    pub table: Option<TableUnion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub conf: i64,
    pub cpe: Option<String>,
    pub method: String,
    pub name: String,
    pub product: Option<String>,
    pub tunnel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub reason: String,
    pub reason_ttl: i64,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scaninfo {
    pub numservices: i64,
    pub protocol: String,
    pub services: String,
    #[serde(rename = "type")]
    pub scaninfo_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Double(f64),
    String(String),
    Integer(i64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptUnion {
    Script(Script),
    ScriptArray(Vec<Script>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsmatchUnion {
    OsmatchElementArray(Vec<Osmatch>),
    Osmatch(Osmatch),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsMatchClassUnion {
    OsclassElement(OsclassElement),
    OsclassElementArray(Vec<OsclassElement>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElemUnion {
    String(String),
    StringArray(Vec<String>),
    Elem(Elem),
    ElemArray(Vec<Elem>),
    ElemUnion(Vec<ElemUnion>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableUnion {
    TableArray(Vec<Table>),
    Table(Box<Table>),
}
