use entity::{ip_host_script, ip_location};
use sea_orm::Set;
use serde::{Deserialize, Serialize};

pub static IP_GEOLOCATION_GEOPLUGIN: &str = "ip-geolocation-geoplugin";
pub static TRACEROUTE_GEOLOCATION: &str = "traceroute-geolocation";

pub fn parse_location(
    ip_main_id: i64,
    scripts: Vec<ip_host_script::ActiveModel>,
) -> anyhow::Result<ip_location::ActiveModel> {
    let mut geoplugin_json: Option<IpGeolocaionGeoplugin> = None;
    let mut traceroute_json: Option<TracerouteGeolocation> = None;

    for script in &scripts {
        if let Some(key) = &script.key {
            if key == IP_GEOLOCATION_GEOPLUGIN {
                geoplugin_json =
                    serde_json::from_str(&script.value.as_deref().unwrap_or_default()).ok();
            } else if key == TRACEROUTE_GEOLOCATION {
                traceroute_json =
                    serde_json::from_str(&script.value.as_deref().unwrap_or_default()).ok();
            }
        }
    }

    let mut model = ip_location::ActiveModel {
        ip_main_id: Set(ip_main_id),
        continent: Set(Default::default()),
        country: Set(geoplugin_json.as_ref().and_then(|g| g.country.clone())),
        country_code: Set(Default::default()),
        state: Set(geoplugin_json.as_ref().and_then(|g| g.region.clone())),
        city: Set(geoplugin_json.and_then(|g| g.city)),
        latitude: Set(traceroute_json.map(|t| t.lat)),
        longitude: Set(traceroute_json.map(|t| t.lon)),
        ..Default::default() // ... set other fields as needed ...
    };

    Ok(model)
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
