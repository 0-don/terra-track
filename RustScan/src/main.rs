mod input;
use input::{Opts, PortRange, ScanOrder};

mod scanner;
use scanner::Scanner;

mod port_strategy;
use port_strategy::PortStrategy;

use cidr_utils::cidr::IpCidr;
use futures::executor::block_on;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, ToSocketAddrs};
use std::path::Path;
use std::time::Duration;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

// Average value for Ubuntu
#[cfg(unix)]
const DEFAULT_FILE_DESCRIPTORS_LIMIT: u64 = 8000;
// Safest batch size based on experimentation
const AVERAGE_BATCH_SIZE: u16 = 3000;

#[cfg(not(tarpaulin_include))]
#[allow(clippy::too_many_lines)]
/// Faster Nmap scanning with Rust
/// If you're looking for the actual scanning, check out the module Scanner
fn main() {
    let opts: Opts = Opts::read();

    println!("Main() `opts` arguments are {:?}", opts);

    let ips: Vec<IpAddr> = parse_addresses(&opts);

    if ips.is_empty() {
        println!("No IPs could be resolved, aborting scan.");
        std::process::exit(1);
    }

    #[cfg(unix)]
    let batch_size: u16 = infer_batch_size(&opts, adjust_ulimit_size(&opts));

    #[cfg(not(unix))]
    let batch_size: u16 = AVERAGE_BATCH_SIZE;

    let scanner = Scanner::new(
        &ips,
        batch_size,
        Duration::from_millis(opts.timeout.into()),
        opts.tries,
        opts.greppable,
        PortStrategy::pick(&opts.range, opts.ports, opts.scan_order),
        opts.accessible,
    );
    println!("Scanner finished building: {:?}", scanner);

    let scan_result = block_on(scanner.run());

    let mut ports_per_ip = HashMap::new();

    for socket in scan_result {
        ports_per_ip
            .entry(socket.ip())
            .or_insert_with(Vec::new)
            .push(socket.port());
    }

    for ip in ips {
        if ports_per_ip.contains_key(&ip) {
            continue;
        }

        // If we got here it means the IP was not found within the HashMap, this
        // means the scan couldn't find any open ports for it.
    }
    // To use the runtime benchmark, run the process as: RUST_LOG=info ./rustscan
}

/// Goes through all possible IP inputs (files or via argparsing)
/// Parses the string(s) into IPs
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

    // If we got to this point this can only be a file path or the wrong input.
    for file_path in unresolved_addresses {
        let file_path = Path::new(file_path);

        if !file_path.is_file() {
            println!("{} is not a file.", file_path.display());

            continue;
        }

        if let Ok(x) = read_ips_from_file(file_path, &backup_resolver) {
            ips.extend(x);
        } else {
            println!("{} is not a valid file.", file_path.display());
        }
    }

    ips
}

/// Given a string, parse it as an host, IP address, or CIDR.
/// This allows us to pass files as hosts or cidr or IPs easily
/// Call this every time you have a possible IP_or_host
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

/// Uses DNS to get the IPS associated with host
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

#[cfg(not(tarpaulin_include))]
/// Parses an input file of IPs and uses those
fn read_ips_from_file(
    ips: &std::path::Path,
    backup_resolver: &Resolver,
) -> Result<Vec<std::net::IpAddr>, std::io::Error> {
    let file = File::open(ips)?;
    let reader = BufReader::new(file);

    let mut ips: Vec<std::net::IpAddr> = Vec::new();

    for address_line in reader.lines() {
        if let Ok(address) = address_line {
            ips.extend(parse_address(&address, backup_resolver));
        } else {
            println!("Line in file is not valid");
        }
    }

    Ok(ips)
}

#[cfg(unix)]
fn adjust_ulimit_size(opts: &Opts) -> u64 {
    use rlimit::Resource;

    if let Some(limit) = opts.ulimit {
        if Resource::NOFILE.set(limit, limit).is_ok() {
            detail!(
                format!("Automatically increasing ulimit value to {limit}."),
                opts.greppable,
                opts.accessible
            );
        } else {
            warning!(
                "ERROR. Failed to set ulimit value.",
                opts.greppable,
                opts.accessible
            );
        }
    }

    let (soft, _) = Resource::NOFILE.get().unwrap();
    soft
}

#[cfg(unix)]
fn infer_batch_size(opts: &Opts, ulimit: u64) -> u16 {
    use std::convert::TryInto;

    let mut batch_size: u64 = opts.batch_size.into();

    // Adjust the batch size when the ulimit value is lower than the desired batch size
    if ulimit < batch_size {
        warning!("File limit is lower than default batch size. Consider upping with --ulimit. May cause harm to sensitive servers",
            opts.greppable, opts.accessible
        );

        // When the OS supports high file limits like 8000, but the user
        // selected a batch size higher than this we should reduce it to
        // a lower number.
        if ulimit < AVERAGE_BATCH_SIZE.into() {
            // ulimit is smaller than aveage batch size
            // user must have very small ulimit
            // decrease batch size to half of ulimit
            warning!("Your file limit is very small, which negatively impacts RustScan's speed. Use the Docker image, or up the Ulimit with '--ulimit 5000'. ", opts.greppable, opts.accessible);
            info!("Halving batch_size because ulimit is smaller than average batch size");
            batch_size = ulimit / 2;
        } else if ulimit > DEFAULT_FILE_DESCRIPTORS_LIMIT {
            info!("Batch size is now average batch size");
            batch_size = AVERAGE_BATCH_SIZE.into();
        } else {
            batch_size = ulimit - 100;
        }
    }
    // When the ulimit is higher than the batch size let the user know that the
    // batch size can be increased unless they specified the ulimit themselves.
    else if ulimit + 2 > batch_size && (opts.ulimit.is_none()) {
        detail!(format!("File limit higher than batch size. Can increase speed by increasing batch size '-b {}'.", ulimit - 100),
        opts.greppable, opts.accessible);
    }

    batch_size
        .try_into()
        .expect("Couldn't fit the batch size into a u16.")
}
