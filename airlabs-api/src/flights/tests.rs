use serde_json as json;

use super::*;

// Example from docs
const FLIGHTS: &str = include_str!("flights_free.json");

#[test]
fn flights() {
    let flights = json::from_str::<Vec<Flights>>(FLIGHTS).unwrap();
    println!("{flights:#?}");
    assert_eq!(flights.len(), 6);
}
