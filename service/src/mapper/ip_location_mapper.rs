use entity::ip_host_script;
use serde::{Deserialize, Serialize};

pub static IP_GEOLOCATION_GEOPLUGIN: &str = "ip-geolocation-geoplugin";
pub static TRACEROUTE_GEOLOCATION: &str = "traceroute-geolocation";

pub fn parse_location(scripts: Vec<ip_host_script::ActiveModel>) -> anyhow::Result<()> {
    let geoplugin = scripts
        .iter()
        .find(|script| script.key.clone().unwrap() == IP_GEOLOCATION_GEOPLUGIN);
    let traceroute = scripts
        .iter()
        .find(|script| script.key.clone().unwrap() == TRACEROUTE_GEOLOCATION);

    if geoplugin.is_some() {
        let geoplugin_json = serde_json::from_str::<IpGeolocaionGeoplugin>(
            &geoplugin.unwrap().value.clone().unwrap().as_str().unwrap(),
        )?;
    }

    if traceroute.is_some() {
        let traceroute_json = serde_json::from_str::<TracerouteGeolocation>(
            &traceroute.unwrap().value.clone().unwrap().as_str().unwrap(),
        )?;
    }

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpGeolocaionGeoplugin {
    #[serde(deserialize_with = "deserialize_nil_as_none")]
    pub city: Option<String>,
    pub region: String,
    pub country: String,
    pub latitute: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TracerouteGeolocation {
    pub ip: String,
    pub hop: f32,
    pub lat: f32,
    pub lon: f32,
    pub rtt: f32,
}

fn deserialize_nil_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref value) if value == "nil" => Ok(None),
        _ => Ok(s),
    }
}
