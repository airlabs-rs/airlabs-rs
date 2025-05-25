use super::*;

mod impls;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlightFree {
    /// ICAO24 Hex address.
    pub hex: Option<String>,
    /// Aircraft Registration Number
    pub reg_number: Option<String>,
    /// Aircraft ICAO type. Available in the Free plan.
    pub aircraft_icao: Option<String>,
    /// ISO 2 country code from Countries DB. Available in the Free plan.
    pub flag: String,
    /// Aircraft Geo-Latitude for now. Available in the Free plan.
    pub lat: Option<f64>,
    /// Aircraft Geo-Longitude for now. Available in the Free plan.
    pub lng: Option<f64>,
    /// Aircraft elevation for now (meters).
    pub alt: Option<u64>,
    /// Aircraft head direction for now. Available in the Free plan.
    pub dir: Option<f64>,
    /// Aircraft horizontal speed (km) for now.
    pub speed: Option<u64>,
    /// Aircraft vertical speed (km) for now.
    pub v_speed: Option<u64>,
    /// Aircraft squawk signal code.
    pub squawk: Option<String>,
    /// Airline IATA code. Available in the Free plan.
    pub airline_iata: String,
    /// Airline ICAO code.
    pub airline_icao: String,
    /// Flight IATA code-number. Available in the Free plan.
    pub flight_iata: String,
    /// Flight ICAO code-number.
    pub flight_icao: String,
    /// Flight number only. Available in the Free plan.
    pub flight_number: String,
    /// Codeshared airline IATA code.
    pub cs_airline_iata: Option<String>,
    /// Codeshared flight IATA code-number.
    pub cs_flight_iata: Option<String>,
    /// Codeshared flight number.
    pub cs_flight_number: Option<String>,
    /// Departure airport IATA code. Available in the Free plan.
    pub dep_iata: String,
    /// Departure airport ICAO code.
    pub dep_icao: String,
    /// Estimated departure terminal.
    pub dep_terminal: Option<String>,
    /// Estimated departure gate.
    pub dep_gate: Option<String>,
    /// Departure time in the airport time zone. Available in the Free plan.
    pub dep_time: String,
    /// Departure UNIX timestamp.
    pub dep_time_ts: u64,
    /// Departure time in UTC time zone.
    pub dep_time_utc: String,
    /// Updated departure time in the airport time zone.
    pub dep_estimated: Option<String>,
    /// Updated departure UNIX timestamp.
    pub dep_estimated_ts: Option<u64>,
    /// Updated departure time in UTC time zone.
    pub dep_estimated_utc: Option<String>,
    /// Arrival airport IATA code. Available in the Free plan.
    pub arr_iata: String,
    /// Arrival airport ICAO code.
    pub arr_icao: String,
    /// Estimated arrival terminal.
    pub arr_terminal: Option<String>,
    /// Estimated arrival gate.
    pub arr_gate: Option<String>,
    /// Arrival baggage claim carousel number.
    pub arr_baggage: Option<String>,
    /// Arrival time in the airport time zone. Available in the Free plan.
    pub arr_time: String,
    /// Arrival UNIX timestamp.
    pub arr_time_ts: u64,
    /// Arrival time in UTC time zone.
    pub arr_time_utc: String,
    /// Updated arrival time in the airport time zone.
    pub arr_estimated: Option<String>,
    /// Updated arrival UNIX timestamp.
    pub arr_estimated_ts: Option<u64>,
    /// Updated arrival time in UTC time zone.
    pub arr_estimated_utc: Option<String>,
    /// Estimated flight time (in minutes).
    pub duration: u64,
    /// (deprecated) Estimated flight delay time (in minutes).
    pub delayed: Option<u64>,
    /// Estimated time of flight departure delay (in minutes).
    pub dep_delayed: Option<u64>,
    /// Estimated time of flight arrival delay (in minutes).
    pub arr_delayed: Option<u64>,
    /// UNIX timestamp of last aircraft signal.
    pub updated: u64,
    /// Current flight status - scheduled, en-route, landed.
    pub status: String,
    /// Aircraft full model name.
    pub model: Option<String>,
    /// Aircraft manufacturer name. Available in the Free plan.
    pub manufacturer: Option<String>,
    /// Manufacturer serial number.
    pub msn: Option<String>,
    /// Aircraft type - landplane, seaplane, tiltrotor, helicopter, gyrocopter, amphibian.
    pub r#type: Option<AircraftType>,
    /// Aircraft engine type - jet, piston, turboprop/turboshaft, electric.
    pub engine: Option<String>,
    /// Aircraft engine number - 1, 2, 3, 4, 6, 8
    pub engine_count: Option<String>,
    /// Aircraft built year
    pub built: Option<u64>,
    /// Aircraft age (years)
    pub age: Option<u64>,

    // Next fields are present in the response, but not documented
    /// Airline name. Available in the Free plan.
    pub airline_name: Option<String>,
    /// Departure airport name
    pub dep_name: Option<String>,
    /// Departure city
    pub dep_city: Option<String>,
    /// Departure country
    pub dep_country: Option<String>,
    /// Actual departure time in the airport time zone.
    pub dep_actual: Option<String>,
    /// Actual departure time UNIX timestamp.
    pub dep_actual_ts: Option<u64>,
    /// Actual departure time in the UTC time zone.
    pub dep_actual_utc: Option<String>,
    /// Arrival airport name
    pub arr_name: Option<String>,
    /// Arrival city
    pub arr_city: Option<String>,
    /// Arrival country
    pub arr_country: Option<String>,
    /// Actual arrival time in the airport time zone.
    pub arr_actual: Option<String>,
    /// Actual arrival time UNIX timestamp.
    pub arr_actual_ts: Option<u64>,
    /// Actual arrival time in the UTC time zone.
    pub arr_actual_utc: Option<String>,
    /// ETA (in minutes).
    pub eta: Option<u64>,
    /// Don't know what this is
    pub percent: Option<u64>,
    /// Response time in UTC timezone
    pub utc: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flight {
    /// ICAO24 Hex address.
    pub hex: Option<String>,
    /// Aircraft Registration Number
    pub reg_number: String,
    /// Aircraft ICAO type. Available in the Free plan.
    pub aircraft_icao: Option<String>,
    /// ISO 2 country code from Countries DB. Available in the Free plan.
    pub flag: String,
    /// Aircraft Geo-Latitude for now. Available in the Free plan.
    pub lat: Option<f64>,
    /// Aircraft Geo-Longitude for now. Available in the Free plan.
    pub lng: Option<f64>,
    /// Aircraft elevation for now (meters).
    pub alt: Option<u64>,
    /// Aircraft head direction for now. Available in the Free plan.
    pub dir: Option<f64>,
    /// Aircraft horizontal speed (km) for now.
    pub speed: Option<u64>,
    /// Aircraft vertical speed (km) for now.
    pub v_speed: Option<u64>,
    /// Aircraft squawk signal code.
    pub squawk: Option<String>,
    /// Airline IATA code. Available in the Free plan.
    pub airline_iata: String,
    /// Airline ICAO code.
    pub airline_icao: String,
    /// Flight IATA code-number. Available in the Free plan.
    pub flight_iata: String,
    /// Flight ICAO code-number.
    pub flight_icao: String,
    /// Flight number only. Available in the Free plan.
    pub flight_number: String,
    /// Codeshared airline IATA code.
    pub cs_airline_iata: Option<String>,
    /// Codeshared flight IATA code-number.
    pub cs_flight_iata: Option<String>,
    /// Codeshared flight number.
    pub cs_flight_number: Option<String>,
    /// Departure airport IATA code. Available in the Free plan.
    pub dep_iata: String,
    /// Departure airport ICAO code.
    pub dep_icao: String,
    /// Estimated departure terminal.
    pub dep_terminal: Option<String>,
    /// Estimated departure gate.
    pub dep_gate: Option<String>,
    /// Departure time in the airport time zone. Available in the Free plan.
    pub dep_time: String,
    /// Departure UNIX timestamp.
    pub dep_time_ts: u64,
    /// Departure time in UTC time zone.
    pub dep_time_utc: String,
    /// Updated departure time in the airport time zone.
    pub dep_estimated: Option<String>,
    /// Updated departure UNIX timestamp.
    pub dep_estimated_ts: Option<u64>,
    /// Updated departure time in UTC time zone.
    pub dep_estimated_utc: Option<String>,
    /// Arrival airport IATA code. Available in the Free plan.
    pub arr_iata: String,
    /// Arrival airport ICAO code.
    pub arr_icao: String,
    /// Estimated arrival terminal.
    pub arr_terminal: Option<String>,
    /// Estimated arrival gate.
    pub arr_gate: Option<String>,
    /// Arrival baggage claim carousel number.
    pub arr_baggage: Option<String>,
    /// Arrival time in the airport time zone. Available in the Free plan.
    pub arr_time: String,
    /// Arrival UNIX timestamp.
    pub arr_time_ts: u64,
    /// Arrival time in UTC time zone.
    pub arr_time_utc: String,
    /// Updated arrival time in the airport time zone.
    pub arr_estimated: Option<String>,
    /// Updated arrival UNIX timestamp.
    pub arr_estimated_ts: Option<u64>,
    /// Updated arrival time in UTC time zone.
    pub arr_estimated_utc: Option<String>,
    /// Estimated flight time (in minutes).
    pub duration: u64,
    /// (deprecated) Estimated flight delay time (in minutes).
    pub delayed: Option<u64>,
    /// Estimated time of flight departure delay (in minutes).
    pub dep_delayed: Option<u64>,
    /// Estimated time of flight arrival delay (in minutes).
    pub arr_delayed: Option<u64>,
    /// UNIX timestamp of last aircraft signal.
    pub updated: u64,
    /// Current flight status - scheduled, en-route, landed.
    pub status: String,
    /// Aircraft full model name.
    pub model: Option<String>,
    /// Aircraft manufacturer name. Available in the Free plan.
    pub manufacturer: Option<String>,
    /// Manufacturer serial number.
    pub msn: Option<String>,
    /// Aircraft type - landplane, seaplane, tiltrotor, helicopter, gyrocopter, amphibian.
    pub r#type: Option<AircraftType>,
    /// Aircraft engine type - jet, piston, turboprop/turboshaft, electric.
    pub engine: String,
    /// Aircraft engine number - 1, 2, 3, 4, 6, 8
    pub engine_count: String,
    /// Aircraft built year
    pub built: Option<u64>,
    /// Aircraft age (years)
    pub age: Option<u64>,

    // Next fields are present in the response, but not documented
    /// Airline name. Available in the Free plan.
    pub airline_name: Option<String>,
    /// Departure airport name
    pub dep_name: Option<String>,
    /// Departure city
    pub dep_city: Option<String>,
    /// Departure country
    pub dep_country: Option<String>,
    /// Actual departure time in the airport time zone.
    pub dep_actual: Option<String>,
    /// Actual departure time UNIX timestamp.
    pub dep_actual_ts: Option<u64>,
    /// Actual departure time in the UTC time zone.
    pub dep_actual_utc: Option<String>,
    /// Arrival airport name
    pub arr_name: Option<String>,
    /// Arrival city
    pub arr_city: Option<String>,
    /// Arrival country
    pub arr_country: Option<String>,
    /// Actual arrival time in the airport time zone.
    pub arr_actual: Option<String>,
    /// Actual arrival time UNIX timestamp.
    pub arr_actual_ts: Option<u64>,
    /// Actual arrival time in the UTC time zone.
    pub arr_actual_utc: Option<String>,
    /// ETA (in minutes).
    pub eta: Option<u64>,
    /// Don't know what this is
    pub percent: Option<u64>,
    /// Response time in UTC timezone
    pub utc: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AircraftType {
    Adsb,
    Landplane,
    Seaplane,
    Tiltrotor,
    Helicopter,
    Gyrocopter,
    Amphibian,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlightRequest {
    api_key: String,
    #[serde(flatten)]
    pub code: FlightCode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlightCode {
    #[serde(rename = "flight_iata")]
    Iata(String),
    #[serde(rename = "flight_icao")]
    Icao(String),
}

impl AirLabsRequest for FlightRequest {
    type Response = Flight;
    type ResponseFree = FlightFree;

    fn url(&self, base: &str) -> String {
        format!("{base}/flight")
    }
}

#[cfg(test)]
mod tests;
