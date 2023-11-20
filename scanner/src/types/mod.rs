use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nmap {
    nmaprun: Nmaprun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nmaprun {
    args: String,
    debugging: Debugging,
    host: Host,
    runstats: Runstats,
    scaninfo: Scaninfo,
    scanner: String,
    start: i64,
    startstr: String,
    taskbegin: Vec<Taskbegin>,
    taskend: Vec<Taskend>,
    taskprogress: Vec<Taskprogress>,
    verbose: Debugging,
    version: f64,
    xmloutputversion: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debugging {
    level: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    address: Address,
    endtime: i64,
    hostnames: Hostnames,
    ports: Ports,
    starttime: i64,
    status: Stat,
    times: Times,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    addr: String,
    addrtype: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hostnames {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    port: Vec<Port>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    portid: i64,
    protocol: Protocol,
    script: ScriptUnion,
    service: Service,
    state: Stat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptUnion {
    PurpleScript(PurpleScript),
    ScriptElementArray(Vec<ScriptElement>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptElement {
    elem: Option<ElemUnion>,
    id: String,
    output: String,
    table: Option<ScriptTable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElemUnion {
    ElemElem(ElemElem),
    ElemElemArray(Vec<ElemElem>),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElemElem {
    key: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptTable {
    IndigoTable(IndigoTable),
    PurpleTableArray(Vec<PurpleTable>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleTable {
    elem: Option<Vec<PurpleElem>>,
    key: TableKey,
    table: Option<TableTableUnion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleElem {
    key: PurpleKey,
    value: PurpleValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PurpleKey {
    Bits,
    #[serde(rename = "commonName")]
    CommonName,
    #[serde(rename = "countryName")]
    CountryName,
    #[serde(rename = "localityName")]
    LocalityName,
    #[serde(rename = "notAfter")]
    NotAfter,
    #[serde(rename = "notBefore")]
    NotBefore,
    #[serde(rename = "organizationName")]
    OrganizationName,
    #[serde(rename = "stateOrProvinceName")]
    StateOrProvinceName,
    Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleValue {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableKey {
    Extensions,
    Issuer,
    Pubkey,
    Subject,
    Validity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableTableUnion {
    FluffyTableArray(Vec<FluffyTable>),
    TentacledTable(TentacledTable),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyTable {
    elem: Vec<FluffyElem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyElem {
    key: FluffyKey,
    value: FluffyValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluffyKey {
    Critical,
    Name,
    Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyValue {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledTable {
    key: String,
    table: StickyTable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyTable {
    elem: Vec<ElemElem>,
    key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoTable {
    elem: Vec<String>,
    key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleScript {
    elem: Vec<ElemElem>,
    id: String,
    output: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    conf: i64,
    method: Method,
    name: String,
    product: Option<String>,
    servicefp: Option<String>,
    tunnel: Option<String>,
    cpe: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Probed,
    Table,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    reason: Reason,
    reason_ttl: i64,
    state: State,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    #[serde(rename = "syn-ack")]
    SynAck,
    #[serde(rename = "user-set")]
    UserSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Open,
    Up,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Times {
    rttvar: i64,
    srtt: i64,
    to: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Runstats {
    finished: Finished,
    hosts: Hosts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finished {
    elapsed: f64,
    exit: String,
    summary: String,
    time: i64,
    timestr: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    down: i64,
    total: i64,
    up: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scaninfo {
    numservices: i64,
    protocol: Protocol,
    services: String,
    #[serde(rename = "type")]
    scaninfo_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taskbegin {
    task: Task,
    time: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Task {
    #[serde(rename = "Connect Scan")]
    ConnectScan,
    #[serde(rename = "NSE")]
    Nse,
    #[serde(rename = "Service scan")]
    ServiceScan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taskend {
    task: Task,
    time: i64,
    extrainfo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taskprogress {
    etc: i64,
    percent: f64,
    remaining: i64,
    task: Task,
    time: i64,
}
