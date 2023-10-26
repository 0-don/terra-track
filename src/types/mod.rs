use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NmapRun {
    scanner: String,
    args: String,
    start: String,
    startstr: String,
    version: String,
    xmloutputversion: String,
    scaninfo: ScanInfo,
    verbose: Verbose,
    debugging: Debugging,
    host: Host,
    runstats: RunStats,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ScanInfo {
    #[serde(rename = "type")]
    scan_type: String,
    protocol: String,
    numservices: String,
    services: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Verbose {
    level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Debugging {
    level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Host {
    starttime: String,
    endtime: String,
    status: Status,
    address: Address,
    hostnames: Hostnames,
    ports: Ports,
    times: Times,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Status {
    state: String,
    reason: String,
    reason_ttl: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Address {
    addr: String,
    addrtype: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Hostnames {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Ports {
    port: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Port {
    protocol: String,
    portid: String,
    state: State,
    service: Service,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct State {
    state: String,
    reason: String,
    reason_ttl: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Service {
    name: String,
    product: Option<String>,
    version: Option<String>,
    extrainfo: Option<String>,
    ostype: Option<String>,
    method: String,
    conf: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Times {
    srtt: String,
    rttvar: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RunStats {
    finished: Finished,
    hosts: Hosts,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Finished {
    time: String,
    timestr: String,
    elapsed: String,
    summary: String,
    exit: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Hosts {
    up: String,
    down: String,
    total: String,
}
