use super::*;

impl Output for api::AirlineFree {
    fn raw(&self) -> String {
        format!("{self:?}")
    }

    fn json(&self) -> serde_json::Result<serde_json::Value> {
        json::to_value(self)
    }

    fn typed(&self) -> json::Result<String> {
        Ok(format!(
            "Name: {}\nIATA: {:?}\nICAO: {:?}",
            self.name, self.iata_code, self.icao_code
        ))
    }
}
