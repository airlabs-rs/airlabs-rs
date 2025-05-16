use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub lang: String,
    pub currency: String,
    pub time: u64,
    pub id: String,
    pub server: String,
    pub host: String,
    pub pid: u64,
    pub key: Key,
    pub params: BTreeMap<String, String>,
    pub version: u64,
    pub method: String,
    pub client: ClientInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    pub id: u64,
    pub api_key: String,
    pub r#type: String,
    pub expired: String,
    pub registered: String,
    pub upgraded: Option<String>,
    pub limits_by_hour: u64,
    pub limits_by_minute: u64,
    pub limits_by_month: u64,
    pub limits_total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub params: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub ip: String,
    pub geo: GeoInfo,
    pub connection: ConnectionInfo,
    pub device: BTreeMap<String, String>,
    pub agent: BTreeMap<String, String>,
    pub karma: Karma,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeoInfo {
    pub country_code: String,
    pub country: String,
    pub continent: String,
    pub city: String,
    pub lat: f64,
    pub lng: f64,
    pub timezone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub r#type: Option<String>,
    pub isp_code: Option<u64>,
    pub isp_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Karma {
    pub is_blocked: bool,
    pub is_crawler: bool,
    pub is_bot: bool,
    pub is_friend: bool,
    pub is_regular: bool,
}
