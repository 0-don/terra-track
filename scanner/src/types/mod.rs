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
    pub ipidsequence: Sequence,
    pub os: Os,
    pub ports: Ports,
    pub starttime: i64,
    pub status: Stat,
    pub tcpsequence: Tcpsequence,
    pub tcptssequence: Sequence,
    pub times: Times,
    pub trace: Trace,
    pub uptime: Uptime,
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
    PurpleElem(PurpleElem),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleElem {
    pub key: String,
    pub value: i64,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub class: String,
    pub values: String,
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
    pub osclass_type: Type,
    pub vendor: Vendor,
    pub osgen: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "general purpose")]
    GeneralPurpose,
    #[serde(rename = "media device")]
    MediaDevice,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "WAP")]
    Wap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Vendor {
    Apple,
    Linux,
    Ruckus,
    Ubiquiti,
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
    pub proto: Proto,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Proto {
    Tcp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Open,
    Up,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    pub port: Vec<Port>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub portid: i64,
    pub protocol: Proto,
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
    Enum(Product),
    FluffyElem(FluffyElem),
    FluffyElemArray(Vec<FluffyElem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Product {
    Cloudflare,
    #[serde(rename = "Cloudflare http proxy")]
    CloudflareHttpProxy,
    Nginx,
    #[serde(rename = "Unbound")]
    Unbound,
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
    pub key: PurpleKey,
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
#[serde(rename_all = "snake_case")]
pub enum PurpleKey {
    #[serde(rename = "Cache_Control")]
    CacheControl,
    #[serde(rename = "Expires")]
    Expires,
    Extensions,
    Issuer,
    Pubkey,
    Subject,
    Validity,
    #[serde(rename = "X_Frame_Options")]
    XFrameOptions,
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
    pub key: FluffyKey,
    pub value: Output,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluffyKey {
    Critical,
    Name,
    Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledTable {
    pub key: TentacledKey,
    pub table: StickyTable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TentacledKey {
    Ecdhparams,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyTable {
    pub elem: Vec<StickyElem>,
    pub key: IndigoKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyElem {
    pub key: StickyKey,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StickyKey {
    Curve,
    Disclosure,
    #[serde(rename = "ec_curve_type")]
    EcCurveType,
    State,
    Title,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndigoKey {
    #[serde(rename = "curve_params")]
    CurveParams,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoTable {
    pub elem: CunningElem,
    pub key: IndecentKey,
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
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum IndecentKey {
    #[serde(rename = "Allowed User Agents")]
    AllowedUserAgents,
    #[serde(rename = "CVE-2007-6750")]
    Cve20076750,
    #[serde(rename = "Strict_Transport_Security")]
    StrictTransportSecurity,
    #[serde(rename = "Supported Methods")]
    SupportedMethods,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentTable {
    pub elem: Option<FriskyElem>,
    pub key: HilariousKey,
    pub table: Option<HilariousTable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyElem {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HilariousKey {
    Dates,
    Description,
    #[serde(rename = "exploit_results")]
    ExploitResults,
    Ids,
    Refs,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousTable {
    pub elem: Vec<PurpleElem>,
    pub key: StickyKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrescriptScript {
    pub id: String,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub conf: i64,
    pub cpe: Option<Cpe>,
    pub method: Method,
    pub name: Name,
    pub product: Option<Product>,
    pub tunnel: Option<Tunnel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cpe {
    #[serde(rename = "cpe:/a:igor_sysoev:nginx")]
    CpeAIgorSysoevNginx,
    #[serde(rename = "cpe:/a:nlnetlabs:unbound")]
    CpeANlnetlabsUnbound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Probed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Name {
    Domain,
    Http,
    Https,
    #[serde(rename = "https-alt")]
    HttpsAlt,
    Tcpwrapped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tunnel {
    Ssl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub reason: Reason,
    pub reason_ttl: i64,
    pub state: State,
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
pub struct Tcpsequence {
    pub difficulty: String,
    pub index: i64,
    pub values: String,
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
pub struct Uptime {
    pub lastboot: String,
    pub seconds: i64,
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
