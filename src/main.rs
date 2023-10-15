mod input;
use input::Opts;
mod scanner;
mod scripts;
use scanner::Scanner;
mod port_strategy;
use crate::scripts::{init_scripts, Script, ScriptFile};
use cidr_utils::cidr::IpCidr;
use futures::executor::block_on;
use port_strategy::PortStrategy;
use std::net::{IpAddr, ToSocketAddrs};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

fn main() {
    // read file as string ./nmap.xml

    // let mut file = File::open("./nmap.xml").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let _results = NmapResults::parse(&contents).unwrap();

    let opts: Opts = Opts::read();
    let ips: Vec<IpAddr> = parse_addresses(&opts);

    let scripts_to_run: Vec<ScriptFile> = init_scripts(opts.scripts).unwrap();

    let scanner = Scanner::new(&ips);

    let ports_per_ip = block_on(scanner.run());

    for (ip, ports) in &ports_per_ip {
        for mut script_f in scripts_to_run.clone() {
            if !opts.command.is_empty() {
                let user_extra_args = &opts.command.join(" ");
                if script_f.call_format.is_some() {
                    let mut call_f = script_f.call_format.unwrap();
                    call_f.push(' ');
                    call_f.push_str(user_extra_args);
                    script_f.call_format = Some(call_f);
                }
            }
            let script = Script::build(
                script_f.path,
                *ip,
                ports.clone(),
                script_f.port,
                script_f.ports_separator,
                script_f.tags,
                script_f.call_format,
            );
            match script.run() {
                Ok(script_result) => {
                    println!("Script result: {}", script_result);
                }
                Err(e) => {
                    println!("Error running script: {}", e);
                }
            }
        }
    }
}

fn parse_addresses(input: &Opts) -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = Vec::new();
    let mut unresolved_addresses: Vec<&str> = Vec::new();
    let backup_resolver =
        Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();

    for address in &input.addresses {
        let parsed_ips = parse_address(address, &backup_resolver);
        if !parsed_ips.is_empty() {
            ips.extend(parsed_ips);
        } else {
            unresolved_addresses.push(address);
        }
    }

    ips
}

fn parse_address(address: &str, resolver: &Resolver) -> Vec<IpAddr> {
    IpCidr::from_str(address)
        .map(|cidr| cidr.iter().collect())
        .ok()
        .or_else(|| {
            format!("{}:{}", &address, 80)
                .to_socket_addrs()
                .ok()
                .map(|mut iter| vec![iter.next().unwrap().ip()])
        })
        .unwrap_or_else(|| resolve_ips_from_host(address, resolver))
}

fn resolve_ips_from_host(source: &str, backup_resolver: &Resolver) -> Vec<IpAddr> {
    let mut ips: Vec<std::net::IpAddr> = Vec::new();

    if let Ok(addrs) = source.to_socket_addrs() {
        for ip in addrs {
            ips.push(ip.ip());
        }
    } else if let Ok(addrs) = backup_resolver.lookup_ip(source) {
        ips.extend(addrs.iter());
    }

    ips
}
