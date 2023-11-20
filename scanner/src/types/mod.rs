use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Nmaprun {
    #[serde(rename = "scaninfo")]
    scaninfo: Scaninfo,

    #[serde(rename = "verbose")]
    verbose: Debugging,

    #[serde(rename = "debugging")]
    debugging: Debugging,

    // #[serde(rename = "taskbegin")]
    // taskbegin: Vec<Taskbegin>,

    // #[serde(rename = "taskend")]
    // taskend: Vec<Taskend>,

    // #[serde(rename = "taskprogress")]
    // taskprogress: Vec<Taskprogress>,

    #[serde(rename = "host")]
    host: Host,

    #[serde(rename = "runstats")]
    runstats: Runstats,

    #[serde(rename = "scanner")]
    scanner: String,

    #[serde(rename = "args")]
    args: String,

    #[serde(rename = "start")]
    start: String,

    #[serde(rename = "startstr")]
    startstr: String,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "xmloutputversion")]
    xmloutputversion: String,
}

#[derive(Serialize, Deserialize)]
pub struct Debugging {
    #[serde(rename = "level")]
    level: String,
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    #[serde(rename = "status")]
    status: Stat,

    #[serde(rename = "address")]
    address: Address,

    #[serde(rename = "hostnames")]
    hostnames: String,

    #[serde(rename = "ports")]
    ports: Ports,

    #[serde(rename = "times")]
    times: Times,

    #[serde(rename = "starttime")]
    starttime: String,

    #[serde(rename = "endtime")]
    endtime: String,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "addr")]
    addr: String,

    #[serde(rename = "addrtype")]
    addrtype: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ports {
    #[serde(rename = "port")]
    port: Vec<Port>,
}

#[derive(Serialize, Deserialize)]
pub struct Port {
    #[serde(rename = "state")]
    state: Stat,

    #[serde(rename = "service")]
    service: Service,

    #[serde(rename = "script")]
    script: ScriptUnion,

    #[serde(rename = "_protocol")]
    protocol: Protocol,

    #[serde(rename = "_portid")]
    portid: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptElement {
    #[serde(rename = "elem")]
    elem: Option<ElemUnion>,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "output")]
    output: String,

    #[serde(rename = "table")]
    table: Option<ScriptTable>,
}

#[derive(Serialize, Deserialize)]
pub struct ElemElement {
    #[serde(rename = "_key")]
    key: String,

    #[serde(rename = "__text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleTable {
    #[serde(rename = "elem")]
    elem: Option<Vec<ElemElement>>,

    #[serde(rename = "_key")]
    key: Key,

    #[serde(rename = "table")]
    table: Option<TableTableUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyTable {
    #[serde(rename = "elem")]
    elem: Vec<ElemElement>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledTable {
    #[serde(rename = "table")]
    table: StickyTable,

    #[serde(rename = "_key")]
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickyTable {
    #[serde(rename = "elem")]
    elem: Vec<ElemElement>,

    #[serde(rename = "_key")]
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoTable {
    #[serde(rename = "elem")]
    elem: Vec<String>,

    #[serde(rename = "_key")]
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleScript {
    #[serde(rename = "elem")]
    elem: Vec<ElemElement>,

    #[serde(rename = "_id")]
    id: String,

    #[serde(rename = "_output")]
    output: String,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "product")]
    product: Option<String>,

    #[serde(rename = "servicefp")]
    servicefp: Option<String>,

    #[serde(rename = "method")]
    method: Method,

    #[serde(rename = "conf")]
    conf: String,

    #[serde(rename = "tunnel")]
    tunnel: Option<String>,

    #[serde(rename = "cpe")]
    cpe: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    #[serde(rename = "state")]
    state: State,

    #[serde(rename = "reason")]
    reason: Reason,

    #[serde(rename = "reason_ttl")]
    reason_ttl: String,
}

#[derive(Serialize, Deserialize)]
pub struct Times {
    #[serde(rename = "srtt")]
    srtt: String,

    #[serde(rename = "rttvar")]
    rttvar: String,

    #[serde(rename = "to")]
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct Runstats {
    #[serde(rename = "finished")]
    finished: Finished,

    #[serde(rename = "hosts")]
    hosts: Hosts,
}

#[derive(Serialize, Deserialize)]
pub struct Finished {
    #[serde(rename = "time")]
    time: String,

    #[serde(rename = "timestr")]
    timestr: String,

    #[serde(rename = "elapsed")]
    elapsed: String,

    #[serde(rename = "summary")]
    summary: String,

    #[serde(rename = "exit")]
    exit: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hosts {
    #[serde(rename = "up")]
    up: String,

    #[serde(rename = "down")]
    down: String,

    #[serde(rename = "total")]
    total: String,
}

#[derive(Serialize, Deserialize)]
pub struct Scaninfo {
    #[serde(rename = "type")]
    scaninfo_type: String,

    #[serde(rename = "protocol")]
    protocol: Protocol,

    #[serde(rename = "numservices")]
    numservices: String,

    #[serde(rename = "services")]
    services: String,
}

#[derive(Serialize, Deserialize)]
pub struct Taskbegin {
    #[serde(rename = "task")]
    task: Task,

    #[serde(rename = "time")]
    time: String,
}

#[derive(Serialize, Deserialize)]
pub struct Taskend {
    #[serde(rename = "task")]
    task: Task,

    #[serde(rename = "time")]
    time: String,

    #[serde(rename = "extrainfo")]
    extrainfo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Taskprogress {
    #[serde(rename = "task")]
    task: Task,

    #[serde(rename = "time")]
    time: String,

    #[serde(rename = "percent")]
    percent: String,

    #[serde(rename = "remaining")]
    remaining: String,

    #[serde(rename = "etc")]
    etc: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptUnion {
    PurpleScript(PurpleScript),

    ScriptElementArray(Vec<ScriptElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElemUnion {
    ElemElement(ElemElement),

    ElemElementArray(Vec<ElemElement>),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptTable {
    IndigoTable(IndigoTable),

    PurpleTableArray(Vec<PurpleTable>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableTableUnion {
    FluffyTableArray(Vec<FluffyTable>),

    TentacledTable(TentacledTable),
}

#[derive(Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "extensions")]
    Extensions,

    #[serde(rename = "issuer")]
    Issuer,

    #[serde(rename = "pubkey")]
    Pubkey,

    #[serde(rename = "subject")]
    Subject,

    #[serde(rename = "validity")]
    Validity,
}

#[derive(Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "probed")]
    Probed,

    #[serde(rename = "table")]
    Table,
}

#[derive(Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "syn-ack")]
    SynAck,

    #[serde(rename = "user-set")]
    UserSet,
}

#[derive(Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,

    #[serde(rename = "up")]
    Up,
}

#[derive(Serialize, Deserialize)]
pub enum Task {
    #[serde(rename = "Connect Scan")]
    ConnectScan,

    #[serde(rename = "NSE")]
    Nse,

    #[serde(rename = "Service scan")]
    ServiceScan,
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
