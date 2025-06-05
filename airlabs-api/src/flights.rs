use super::*;

mod impls;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flights {
    /// ICAO24 Hex address.
    pub hex: Option<String>,
    /// Aircraft Registration Number
    pub reg_number: Option<String>,
    /// ISO 2 country code from Countries DB. Available in the Free plan.
    pub flag: String,
    /// Aircraft Geo-Latitude for now. Available in the Free plan.
    pub lat: Option<f64>,
    /// Aircraft Geo-Longitude for now. Available in the Free plan.
    pub lng: Option<f64>,
    /// Aircraft elevation for now (meters).
    pub alt: Option<i64>,
    /// Aircraft head direction for now. Available in the Free plan.
    pub dir: Option<f64>,
    /// Aircraft horizontal speed (km) for now.
    pub speed: Option<u64>,
    /// Aircraft vertical speed (km) for now.
    pub v_speed: Option<u64>,
    /// Aircraft squawk signal code.
    pub squawk: Option<String>,
    /// Airline ICAO code.
    pub airline_icao: String,
    /// Airline IATA code. Available in the Free plan.
    pub airline_iata: Option<String>,
    /// Aircraft ICAO type. Available in the Free plan.
    pub aircraft_icao: Option<String>,
    /// Flight ICAO code-number.
    pub flight_icao: Option<String>,
    /// Flight IATA code-number. Available in the Free plan.
    pub flight_iata: Option<String>,
    /// Flight number only. Available in the Free plan.
    pub flight_number: Option<String>,
    /// Departure airport ICAO code.
    pub dep_icao: Option<String>,
    /// Departure airport IATA code. Available in the Free plan.
    pub dep_iata: Option<String>,
    /// Arrival airport ICAO code.
    pub arr_icao: Option<String>,
    /// Arrival airport IATA code. Available in the Free plan.
    pub arr_iata: Option<String>,
    /// UNIX timestamp of last aircraft signal.
    pub updated: u64,
    /// Current flight status - scheduled, en-route, landed.
    pub status: FlightStatus,

    // Next fields are present in the response, but not documented
    /// Aircraft type - landplane, seaplane, tiltrotor, helicopter, gyrocopter, amphibian.
    pub r#type: Option<AircraftType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlightsRequest {
    api_key: String,

    /// Bounding box (South-West Lat, South-West Long, North-East Lat, North-East Long)
    pub bbox: Option<BoundingBox>,
    /// Map zoom level to reduce the number of flights to speed up rendering (0-11)
    pub zoom: Option<u8>,
    /// Filtering by ICAO24 Hex address.
    pub hex: Option<String>,
    /// Filtering by aircraft Registration number.
    pub reg_number: Option<String>,
    /// Filtering by Airline ICAO code.
    pub airline_icao: Option<String>,
    /// Filtering by Airline IATA code.
    pub airline_iata: Option<String>,
    /// Filtering by Airline Country ISO 2 code from Countries DB.
    pub flag: Option<String>,
    /// Filtering by Flight ICAO code-number.
    pub flight_icao: Option<String>,
    /// Filtering by Flight IATA code-number.
    pub flight_iata: Option<String>,
    /// Filtering by Flight number only.
    pub flight_number: Option<String>,
    /// Filtering by departure Airport ICAO code.
    pub dep_icao: Option<String>,
    /// Filtering by departure Airport IATA code.
    pub dep_iata: Option<String>,
    /// Filtering by arrival Airport ICAO code.
    pub arr_icao: Option<String>,
    /// Filtering by arrival Airport IATA code.
    pub arr_iata: Option<String>,
    /// Fields to return (comma separated, e.g.: hex,airline_iata,lat,lng)
    pub _fields: Option<String>,
    /// View format - object (default) or array (good for browser)
    pub _view: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundingBox(
    /// south_west_lat
    f64,
    /// south_west_long
    f64,
    /// north_east_lat
    f64,
    /// north_east_long
    f64,
);

impl AirLabsRequest for FlightsRequest {
    type Response = Vec<Flights>;
    type ResponseFree = Vec<Flights>;
    const METHOD: &'static str = "flights";
}

#[cfg(test)]
mod tests;
