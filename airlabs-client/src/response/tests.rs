use super::*;

const AIRLINES_RESPONSE: &str = include_str!("airlines_free.json");
const AIRPORTS_RESPONSE: &str = include_str!("airports_free.json");

#[test]
fn airlines() {
    let now = time::Instant::now();
    let response = Response::new(AIRLINES_RESPONSE, now.elapsed());
    let response = response.api_response::<Vec<api::AirlineFree>>().unwrap();
    let airlines = response.into_result().unwrap();
    assert_eq!(airlines.len(), 6500);
}

#[test]
fn airports() {
    let now = time::Instant::now();
    let response = Response::new(AIRPORTS_RESPONSE, now.elapsed());
    let response = response.api_response::<Vec<api::AirportFree>>().unwrap();
    let airlines = response.into_result().unwrap();
    assert_eq!(airlines.len(), 6);
}
