use serde_json as json;

use super::*;

// Example from docs
const FLIGHT: &str = include_str!("docs.json");

const FLIGHT_FREE: &str = include_str!("aa6.json");

const EK96_FREE: &str = include_str!("ek96.json");

#[test]
fn flight() {
    let flight = json::from_str::<Flight>(FLIGHT).unwrap();
    println!("{flight:#?}");
    assert_eq!(flight.r#type, Some(AircraftType::Landplane));
}

#[test]
fn flight_free() {
    let flight = json::from_str::<FlightFree>(FLIGHT_FREE).unwrap();
    println!("{flight:#?}");
    assert_eq!(flight.r#type, None);
}

#[test]
fn ek96_free() {
    let flight = json::from_str::<FlightFree>(EK96_FREE).unwrap();
    println!("{flight:#?}");
    assert_eq!(flight.r#type, Some(AircraftType::Adsb));
}

#[test]
fn flight_iata() {
    let req = FlightRequest::iata("KL1606");
    let req = json::to_string(&req).unwrap();
    assert_eq!(req, r#"{"flight_iata":"KL1606"}"#);
}

#[test]
fn flight_icao() {
    let req = FlightRequest::icao("KLM1606");
    let req = json::to_string(&req).unwrap();
    assert_eq!(req, r#"{"flight_icao":"KLM1606"}"#);
}
