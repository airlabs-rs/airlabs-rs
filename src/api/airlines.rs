use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Airline {
    /// Public name. Available in the Free plan.
    pub name: String,
    /// 3 Numeric Airline Prefix Code for Cargo Operations​.
    pub iata_prefix: u64,
    /// 3 Numeric Accounting Code for Passenger Operations.
    pub iata_accounting: u64,
    /// Official IATA code. Available in the Free plan.
    pub iata_code: String,
    /// Official ICAO code. Available in the Free plan.
    pub icao_code: String,
    /// Allocated ICAO callsign.
    pub callsign: String,
    /// ISO 2 country code from Countries DB.
    pub country_code: String,
    /// IATA Operational Safety Audit Program.
    pub iosa_registered: u64,
    /// The airline provides scheduled flights.
    pub is_scheduled: u64,
    /// The airline provides passenger services.
    pub is_passenger: u64,
    /// The airline provides cargo services.
    pub is_cargo: u64,
    /// The airline provides international flights.
    pub is_international: u64,
    /// Confirmed total aircrafts.
    pub total_aircrafts: u64,
    /// Average airline fleet age (years).
    pub average_fleet_age: u64,
    /// Airline accidents for the last 5 years.
    pub accidents_last_5y: u64,
    /// Airline crashes for the last 5 years.
    pub crashes_last_5y: u64,
    /// Airline official website.
    pub website: String,
    /// Airline official Facebook page.
    pub facebook: String,
    /// Airline official Twitter account.
    pub twitter: String,
    /// Airline official Instagram profile.
    pub instagram: String,
    /// Airline official Linkedin profile.
    pub linkedin: String,
    /// AirLabs generated uniq Airline ID, which you can use for SEO, etc.
    pub slug: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AirlinesRequest {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_accounting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callsign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _fields: Option<String>,
}

impl AirlinesRequest {
    pub fn new(key: &str) -> Self {
        let api_key = key.to_string();
        Self {
            api_key,
            ..default()
        }
    }
}

impl AirLabsRequest for AirlinesRequest {
    fn url(&self, base: &str) -> String {
        format!("{base}/airlines")
    }
}
