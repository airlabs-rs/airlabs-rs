use super::*;

impl Output for api::Airline {
    fn output(&self) -> String {
        format!(
            "Name: {}\n IATA:   {:?}\n ICAO:   {:?}\n Callsign: {}",
            self.name, self.iata_code, self.icao_code, self.callsign
        )
    }
}

impl Output for api::AirlineFree {
    fn output(&self) -> String {
        format!(
            "Name: {}\n IATA: {:?}\n ICAO: {:?}",
            self.name, self.iata_code, self.icao_code
        )
    }
}
