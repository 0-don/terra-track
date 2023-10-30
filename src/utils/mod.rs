// use crate::{scanner::Scanner, scripts::Script, types::NmapXML};
// use std::net::IpAddr;
// pub mod macros;

// //ignore linting
// #[allow(dead_code)]
// pub async fn scan(ip: &'static str) -> anyhow::Result<NmapXML> {
//     let ip: IpAddr = ip.parse()?;
//     let ports = Scanner::new(ip).run().await?;

//     println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

//     let script = Script::new(ip, ports);
//     let result = script.run();
//     if let Ok(result) = result {
//         return Ok(result);
//     }

//     Err(result.err().unwrap())
// }
