use super::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlightStatus {
    Scheduled,
    #[serde(rename = "en-route")]
    EnRoute,
    Landed,
}
