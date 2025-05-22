use super::*;

impl Output for api::Airport {
    fn output(&self) -> String {
        format!(
            "Name: {}\n IATA:   {:?}\n ICAO:   {:?}\n City: {}",
            self.name, self.iata_code, self.icao_code, self.city
        )
    }
}

impl Output for api::AirportFree {
    fn output(&self) -> String {
        format!(
            "Name: {}\n IATA: {:?}\n ICAO: {:?}",
            self.name, self.iata_code, self.icao_code
        )
    }
}
