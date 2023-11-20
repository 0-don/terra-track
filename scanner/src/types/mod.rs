use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nmap {
    pub nmaprun: Nmaprun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nmaprun {
    pub args: String,
    pub host: Host,
    pub runstats: Runstats,
    pub scaninfo: Scaninfo,
    pub scanner: String,
    pub start: i64,
    pub startstr: String,
    pub taskbegin: Vec<Taskbegin>,
    pub taskend: Vec<Taskend>,
    pub taskprogress: Vec<Taskprogress>,
    pub version: f64,
    pub xmloutputversion: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub address: Address,
    pub endtime: i64,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Stat,
    pub times: Times,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub addr: String,
    pub addrtype: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    pub port: Vec<Port>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub portid: i64,
    pub protocol: String,
    pub script: ScriptUnion,
    pub service: Service,
    pub state: Stat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptUnion {
    PurpleScript(PurpleScript),
    ScriptElementArray(Vec<ScriptElement>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptElement {
    pub elem: Option<ElemUnion>,
    pub id: String,
    pub output: String,
    pub table: Option<ScriptTable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElemUnion {
    ElemElem(PurpleElem),
    ElemElemArray(Vec<PurpleElem>),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptTable {
    IndigoTable(IndigoTable),
    PurpleTableArray(Vec<PurpleTable>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleTable {
    pub elem: Option<Vec<PurpleElem>>,
    pub key: String,
    pub table: Option<TableTableUnion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleElem {
    pub key: String,
    pub value: PurpleValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleValue {
    Integer(i64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableTableUnion {
    FluffyTableArray(Vec<FluffyTable>),
    TentacledTable(TentacledTable),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyTable {
    pub elem: Vec<PurpleElem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledTable {
    pub key: String,
    pub table: StickyTable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyTable {
    pub elem: Vec<PurpleElem>,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoTable {
    pub elem: Vec<String>,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleScript {
    pub elem: Vec<PurpleElem>,
    pub id: String,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub conf: i64,
    pub method: String,
    pub name: String,
    pub product: Option<String>,
    pub ostype: Option<String>,
    pub servicefp: Option<String>,
    pub extrainfo: Option<String>,
    pub version: Option<String>,
    pub tunnel: Option<String>,
    pub cpe: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub reason: String,
    pub reason_ttl: i64,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Times {
    pub rttvar: i64,
    pub srtt: i64,
    pub to: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Runstats {
    pub finished: Finished,
    pub hosts: Hosts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finished {
    pub elapsed: f64,
    pub exit: String,
    pub summary: String,
    pub time: i64,
    pub timestr: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    pub down: i64,
    pub total: i64,
    pub up: i64,
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
pub struct Taskbegin {
    pub task: String,
    pub time: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taskend {
    pub task: String,
    pub time: i64,
    pub extrainfo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taskprogress {
    pub etc: i64,
    pub percent: f64,
    pub remaining: i64,
    pub task: String,
    pub time: i64,
}
