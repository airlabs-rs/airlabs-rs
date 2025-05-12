use super::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight {
    /// ICAO24 Hex address
    pub hex: Option<String>,
    /// Aircraft Registration Number
    pub reg_number: Option<String>,
    /// Aircraft ICAO type
    pub aircraft_icao: Option<String>,
    /// ISO 2 country code
    pub flag: Option<String>,
    /// Aircraft Geo-Latitude
    pub lat: Option<f64>,
    /// Aircraft Geo-Longitude
    pub lng: Option<f64>,
    /// Aircraft elevation (meters)
    pub alt: Option<f64>,
    /// Aircraft head direction
    pub dir: Option<f64>,
    /// Aircraft horizontal speed (km/h)
    pub speed: Option<f64>,
    /// Aircraft vertical speed (km/h)
    pub v_speed: Option<f64>,
    /// Aircraft squawk signal code
    pub squawk: Option<String>,
    /// Airline IATA code
    pub airline_iata: Option<String>,
    /// Airline ICAO code
    pub airline_icao: Option<String>,
    /// Flight IATA code-number
    pub flight_iata: Option<String>,
    /// Flight ICAO code-number
    pub flight_icao: Option<String>,
    /// Flight number only
    pub flight_number: Option<String>,
    /// Codeshared airline IATA code
    pub cs_airline_iata: Option<String>,
    /// Codeshared flight IATA code-number
    pub cs_flight_iata: Option<String>,
    /// Codeshared flight number
    pub cs_flight_number: Option<String>,
    /// Departure airport IATA code
    pub dep_iata: Option<String>,
    /// Departure airport ICAO code
    pub dep_icao: Option<String>,
    /// Estimated departure terminal
    pub dep_terminal: Option<String>,
    /// Estimated departure gate
    pub dep_gate: Option<String>,
    /// Departure time in airport time zone
    pub dep_time: Option<String>,
    /// Departure UNIX timestamp
    pub dep_time_ts: Option<i64>,
    /// Departure time in UTC
    pub dep_time_utc: Option<String>,
    /// Updated departure time in airport time zone
    pub dep_estimated: Option<String>,
    /// Updated departure UNIX timestamp
    pub dep_estimated_ts: Option<i64>,
    /// Updated departure time in UTC
    pub dep_estimated_utc: Option<String>,
    /// Arrival airport IATA code
    pub arr_iata: Option<String>,
    /// Arrival airport ICAO code
    pub arr_icao: Option<String>,
    /// Estimated arrival terminal
    pub arr_terminal: Option<String>,
    /// Estimated arrival gate
    pub arr_gate: Option<String>,
    /// Arrival baggage claim carousel number
    pub arr_baggage: Option<String>,
    /// Arrival time in airport time zone
    pub arr_time: Option<String>,
    /// Arrival UNIX timestamp
    pub arr_time_ts: Option<i64>,
    /// Arrival time in UTC
    pub arr_time_utc: Option<String>,
    /// Updated arrival time in airport time zone
    pub arr_estimated: Option<String>,
    /// Updated arrival UNIX timestamp
    pub arr_estimated_ts: Option<i64>,
    /// Updated arrival time in UTC
    pub arr_estimated_utc: Option<String>,
    /// Estimated flight time (minutes)
    pub duration: Option<i32>,
    /// (deprecated) Estimated flight delay time (minutes)
    pub delayed: Option<i32>,
    /// Estimated departure delay (minutes)
    pub dep_delayed: Option<i32>,
    /// Estimated arrival delay (minutes)
    pub arr_delayed: Option<i32>,
    /// UNIX timestamp of last aircraft signal
    pub updated: Option<i64>,
    /// Current flight status
    pub status: Option<String>,
    /// Aircraft full model name
    pub model: Option<String>,
    /// Aircraft manufacturer name
    pub manufacturer: Option<String>,
    /// Manufacturer serial number
    pub msn: Option<String>,
    /// Aircraft type - landplane, seaplane, tiltrotor, helicopter, gyrocopter, amphibian
    #[serde(rename = "type")]
    pub aircraft_type: Option<String>,
    /// Aircraft engine type - jet, piston, turboprop/turboshaft, electric
    pub engine: Option<String>,
    /// Aircraft engine number - 1, 2, 3, 4, 6, 8
    pub engine_count: Option<String>,
    /// Aircraft built year
    pub built: Option<i32>,
    /// Aircraft age (years)
    pub age: Option<i32>,
}

#[derive(Debug)]
pub struct FlightQuery {
    pub flight_icao: Option<String>,
    pub flight_iata: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlightRequest {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_icao: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_iata: Option<String>,
}


impl FlightRequest {
    pub fn new(key: &str, flight_query: FlightQuery) -> Self {
        let api_key = key.to_string();
        Self{
            api_key,
            flight_iata: flight_query.flight_iata,
            flight_icao: flight_query.flight_icao,
        }
    }
}

impl AirLabsRequest for FlightRequest {
    fn url(&self, base: &str) -> String {
        format!("{base}/flight")
    }
}
