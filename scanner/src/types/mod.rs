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
    pub endtime: i64,
    pub hostnames: Hostnames,
    pub os: Os,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Stat,
    pub times: Times,
    pub trace: Trace,
    pub hostscript: Option<Hostscript>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub addr: String,
    pub addrtype: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hostnames {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hostscript {
    pub script: Vec<HostscriptScript>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostscriptScript {
    pub elem: Option<Vec<IndigoElem>>,
    pub id: String,
    pub output: String,
    pub table: Option<ScriptTableClass>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoElem {
    FluffyElem(FluffyElem),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptTableClass {
    pub elem: Vec<FluffyElem>,
    pub key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyElem {
    pub key: String,
    pub value: Value,
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
pub struct Os {
    pub osfingerprint: Osfingerprint,
    pub osmatch: OsmatchUnion,
    pub portused: Portused,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Osfingerprint {
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsmatchUnion {
    OsmatchElementArray(Vec<OsmatchElement>),
    PurpleOsmatch(PurpleOsmatch),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsmatchElement {
    pub accuracy: i64,
    pub line: i64,
    pub name: String,
    pub osclass: OsclassUnion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OsclassUnion {
    OsclassElement(OsclassElement),
    OsclassElementArray(Vec<OsclassElement>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsclassElement {
    pub accuracy: i64,
    pub cpe: String,
    pub osfamily: String,
    #[serde(rename = "type")]
    pub osclass_type: String,
    pub vendor: String,
    pub osgen: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleOsmatch {
    pub accuracy: i64,
    pub line: i64,
    pub name: String,
    pub osclass: OsclassElement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Portused {
    pub portid: i64,
    pub proto: String,
    pub state: String,
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
#[serde(untagged)]
pub enum ScriptUnion {
    PrescriptScript(PrescriptScript),
    PurpleScriptArray(Vec<PurpleScript>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleScript {
    pub elem: Option<IndecentElem>,
    pub id: String,
    pub output: Output,
    pub table: Option<ScriptTableUnion>,
    pub value: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentElem {
    Enum(String),
    FluffyElem(FluffyElem),
    FluffyElemArray(Vec<FluffyElem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Output {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptTableUnion {
    IndigoTable(IndigoTable),
    PurpleTableArray(Vec<PurpleTable>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleTable {
    pub elem: Option<HilariousElem>,
    pub key: String,
    pub table: Option<TableTableUnion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousElem {
    String(String),
    UnionArray(Vec<AmbitiousElem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousElem {
    FluffyElem(FluffyElem),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableTableUnion {
    FluffyTableArray(Vec<FluffyTable>),
    TentacledTable(TentacledTable),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyTable {
    pub elem: Vec<TentacledElem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledElem {
    pub key: String,
    pub value: Output,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledTable {
    pub key: String,
    pub table: StickyTable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyTable {
    pub elem: Vec<StickyElem>,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyElem {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoTable {
    pub elem: CunningElem,
    pub key: String,
    pub table: Option<Vec<IndecentTable>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningElem {
    String(String),
    UnionArray(Vec<MagentaElem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaElem {
    StickyElem(StickyElem),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentTable {
    pub elem: Option<FriskyElem>,
    pub key: String,
    pub table: Option<HilariousTable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyElem {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousTable {
    pub elem: Vec<FluffyElem>,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrescriptScript {
    pub id: String,
    pub output: String,
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
pub struct Times {
    pub rttvar: i64,
    pub srtt: i64,
    pub to: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    pub hop: Hop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hop {
    pub ipaddr: String,
    pub rtt: f64,
    pub ttl: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scaninfo {
    pub numservices: i64,
    pub protocol: String,
    pub services: String,
    #[serde(rename = "type")]
    pub scaninfo_type: String,
}
