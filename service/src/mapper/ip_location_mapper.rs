use entity::{ip_host_script, ip_location};
use sea_orm::Set;
use serde::{Deserialize, Serialize};

pub static IP_GEOLOCATION_GEOPLUGIN: &str = "ip-geolocation-geoplugin";
pub static TRACEROUTE_GEOLOCATION: &str = "traceroute-geolocation";

pub fn parse_location(
    ip_main_id: &i64,
    scripts: &Vec<ip_host_script::ActiveModel>,
) -> anyhow::Result<ip_location::ActiveModel> {
    let mut geoplugin_json: Option<IpGeolocaionGeoplugin> = None;
    let mut traceroute_json: Option<TracerouteGeolocation> = None;

    for script in scripts {
        let key = script.key.clone().unwrap();
        if key == IP_GEOLOCATION_GEOPLUGIN {
            if let sea_orm::ActiveValue::Set(value) = &script.value {
                if let Ok(parsed) =
                    serde_json::from_str::<IpGeolocaionGeoplugin>(value.to_string().as_str())
                {
                    geoplugin_json = Some(parsed);
                }
            }
        } else if key == TRACEROUTE_GEOLOCATION {
            if let sea_orm::ActiveValue::Set(value) = &script.value {
                if let Ok(parsed) =
                    serde_json::from_str::<TracerouteGeolocation>(value.to_string().as_str())
                {
                    traceroute_json = Some(parsed);
                }
            }
        }
    }

    if geoplugin_json.is_none() && traceroute_json.is_none() {
        return Err(anyhow::anyhow!("No location found"));
    }

    let model = ip_location::ActiveModel {
        ip_main_id: Set(ip_main_id.clone()),
        continent: Set(None),
        country: Set(geoplugin_json
            .as_ref()
            .and_then(|g| Some(g.country.clone()))),
        country_code: Set(None),
        state: Set(geoplugin_json.as_ref().and_then(|g| Some(g.region.clone()))),
        city: Set(geoplugin_json.and_then(|g| g.city)),
        latitude: Set(traceroute_json.as_ref().map(|t| t.lat)),
        longitude: Set(traceroute_json.map(|t| t.lon)),
        postal: Set(None),
        timezone: Set(None),
        ..Default::default()
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
